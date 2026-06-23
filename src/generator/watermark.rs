use std::error::Error;

use lopdf::{
    Object,
    Document,
    ObjectId,
    Dictionary,

    content::{
        Content,
        Operation
    },
};

use crate::{
    syntax::vars::Vars,

    ui::{
        ui_base::UI,
        errors_alerts::ErrorsAlerts,
        success_alerts::SuccessAlerts,
    },
};

pub struct Watermark {
    contents: String,
}

impl Watermark {

    pub fn new(contents: &str) -> Self {
        Self {
            contents: contents.to_string(),
        }
    }

    pub fn get(&self) {
        let jobs = Vars.get_all_watermark(&self.contents);

        if jobs.is_empty() {
            return;
        }

        UI::section_header("Watermarking PDFs", "normal");

        for (input, mode, content, output) in jobs {
            let result = if mode == "image" {
                self.image(&input, &content, &output)
            } else {
                self.text(&input, &content, &output)
            };

            match result {
                Ok(count) => SuccessAlerts::watermarked(&output, &mode, count),
                Err(e) => ErrorsAlerts::generic(&e.to_string()),
            }
        }
    }

    fn text(&self, input: &str, text: &str, output: &str) -> Result<usize, Box<dyn Error>> {
        let mut doc = Document::load(input)?;

        let font_id = doc.add_object(Self::font());
        let gs_id = doc.add_object(Self::alpha_state());

        let page_ids: Vec<ObjectId> = doc.get_pages().values().copied().collect();

        if page_ids.is_empty() {
            return Err(format!("No pages found in: {}", input).into());
        }

        let count = page_ids.len();
        for page_id in page_ids {
            let (width, height) = Self::dimensions(&doc, page_id);

            Self::add_resources(&mut doc, page_id, &[
                (b"Font", b"WMFont", font_id),
                (b"ExtGState", b"WMgs", gs_id),
            ]);

            // Light-gray text, ~45° across the page, drawn semi-transparently.
            let operations = vec![
                Operation::new("q", vec![]),
                Operation::new("gs", vec![Object::Name(b"WMgs".to_vec())]),
                Operation::new("rg", vec![Object::Real(0.6), Object::Real(0.6), Object::Real(0.6)]),
                Operation::new("BT", vec![]),
                Operation::new("Tf", vec![Object::Name(b"WMFont".to_vec()), Object::Integer(60)]),
                Operation::new("Tm", vec![
                    Object::Real(0.7071), Object::Real(0.7071),
                    Object::Real(-0.7071), Object::Real(0.7071),
                    Object::Real(width * 0.2), Object::Real(height * 0.35),
                ]),
                Operation::new("Tj", vec![Object::string_literal(text)]),
                Operation::new("ET", vec![]),
                Operation::new("Q", vec![]),
            ];

            doc.add_to_page_content(page_id, Content { operations })?;
        }

        doc.save(output)?;
        Ok(count)
    }

    fn image(&self, input: &str, image_path: &str, output: &str) -> Result<usize, Box<dyn Error>> {
        let mut doc = Document::load(input)?;

        let stamp = lopdf::xobject::image(image_path)?;
        let img_w = stamp.dict.get(b"Width").and_then(|o| o.as_i64()).unwrap_or(1) as f32;
        let img_h = stamp.dict.get(b"Height").and_then(|o| o.as_i64()).unwrap_or(1) as f32;

        let img_id = doc.add_object(stamp);
        let gs_id = doc.add_object(Self::alpha_state());

        let page_ids: Vec<ObjectId> = doc.get_pages().values().copied().collect();

        if page_ids.is_empty() {
            return Err(format!("No pages found in: {}", input).into());
        }

        let count = page_ids.len();

        for page_id in page_ids {
            let (width, height) = Self::dimensions(&doc, page_id);

            // Scale to ~40% of the page width, keeping aspect ratio, centered.
            let draw_w = width * 0.4;
            let draw_h = img_h * (draw_w / img_w);
            let x = (width - draw_w) / 2.0;
            let y = (height - draw_h) / 2.0;

            Self::add_resources(&mut doc, page_id, &[
                (b"XObject", b"WMImg", img_id),
                (b"ExtGState", b"WMgs", gs_id),
            ]);

            let operations = vec![
                Operation::new("q", vec![]),
                Operation::new("gs", vec![Object::Name(b"WMgs".to_vec())]),
                Operation::new("cm", vec![
                    Object::Real(draw_w), Object::Real(0.0),
                    Object::Real(0.0), Object::Real(draw_h),
                    Object::Real(x), Object::Real(y),
                ]),
                Operation::new("Do", vec![Object::Name(b"WMImg".to_vec())]),
                Operation::new("Q", vec![]),
            ];

            doc.add_to_page_content(page_id, Content { operations })?;
        }

        doc.save(output)?;
        Ok(count)
    }

    fn font() -> Dictionary {
        let mut font = Dictionary::new();
        font.set("Type", Object::Name(b"Font".to_vec()));
        font.set("Subtype", Object::Name(b"Type1".to_vec()));
        font.set("BaseFont", Object::Name(b"Helvetica".to_vec()));
        font
    }

    fn alpha_state() -> Dictionary {
        let mut gs = Dictionary::new();
        gs.set("Type", Object::Name(b"ExtGState".to_vec()));
        gs.set("ca", Object::Real(0.3));
        gs.set("CA", Object::Real(0.3));
        gs
    }

    // Adds named resources to a page, merging into existing (possibly shared)
    // resource sub-dictionaries so the page's own fonts/xobjects are preserved.
    fn add_resources(doc: &mut Document, page_id: ObjectId, entries: &[(&[u8], &[u8], ObjectId)]) {
        let mut resources = Self::resources_owned(doc, page_id);

        for (category, name, target) in entries {
            let mut sub: Dictionary = match resources.get(category) {
                Ok(Object::Dictionary(dict)) => dict.clone(),
                Ok(Object::Reference(id)) => doc.get_dictionary(*id).map(|dict| dict.clone()).unwrap_or_else(|_| Dictionary::new()),
                _ => Dictionary::new(),
            };

            sub.set(name.to_vec(), Object::Reference(*target));
            resources.set(category.to_vec(), Object::Dictionary(sub));
        }

        if let Ok(page) = doc.get_dictionary_mut(page_id) {
            page.set("Resources", Object::Dictionary(resources));
        }
    }

    fn resources_owned(doc: &Document, page_id: ObjectId) -> Dictionary {
        let mut reference = None;

        if let Ok(page) = doc.get_dictionary(page_id) {
            match page.get(b"Resources") {
                Ok(Object::Dictionary(dict)) => return dict.clone(),
                Ok(Object::Reference(id)) => reference = Some(*id),
                _ => {}
            }
        }

        reference
            .and_then(|id| doc.get_dictionary(id).ok().cloned())
            .unwrap_or_else(Dictionary::new)
    }

    fn dimensions(doc: &Document, page_id: ObjectId) -> (f32, f32) {
        if let Ok(page) = doc.get_dictionary(page_id) {
            if let Ok(Object::Array(media_box)) = page.get(b"MediaBox") {
                if media_box.len() == 4 {
                    let at = |i: usize| {
                        media_box[i].as_float()
                            .or_else(|_| media_box[i].as_i64().map(|v| v as f32))
                            .unwrap_or(0.0)
                    };

                    let (w, h) = (at(2) - at(0), at(3) - at(1));

                    if w > 0.0 && h > 0.0 {
                        return (w, h);
                    }
                }
            }
        }

        // A4 fallback when the page inherits its MediaBox from the page tree.
        (595.0, 842.0)
    }

}
