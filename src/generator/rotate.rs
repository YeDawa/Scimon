use std::error::Error;

use lopdf::{
    Document,
    ObjectId,
};

use crate::{
    syntax::vars::Vars,

    ui::{
        ui_base::UI,
        errors_alerts::ErrorsAlerts,
        success_alerts::SuccessAlerts,
    },
};

pub struct Rotate {
    contents: String,
}

impl Rotate {

    pub fn new(contents: &str) -> Self {
        Self {
            contents: contents.to_string(),
        }
    }

    pub fn get(&self) {
        let jobs = Vars.get_all_rotate(&self.contents);

        if jobs.is_empty() {
            return;
        }

        UI::section_header("Rotating PDFs", "normal");

        for (input, angle, output) in jobs {
            match self.rotate_one(&input, angle, &output) {
                Ok(count) => SuccessAlerts::rotated(&output, angle, count),
                Err(e) => ErrorsAlerts::generic(&e.to_string()),
            }
        }
    }

    pub fn rotate_one(&self, input: &str, angle: i64, output: &str) -> Result<usize, Box<dyn Error>> {
        if angle % 90 != 0 {
            return Err(format!("Rotation must be a multiple of 90 degrees: {}", angle).into());
        }

        let mut doc = Document::load(input)?;
        let page_ids: Vec<ObjectId> = doc.get_pages().values().copied().collect();

        if page_ids.is_empty() {
            return Err(format!("No pages found in: {}", input).into());
        }

        let count = page_ids.len();

        for page_id in page_ids {
            // `/Rotate` is added to the page's current rotation and normalized
            // into the 0..360 range PDF viewers expect.
            let current = doc.get_dictionary(page_id)
                .ok()
                .and_then(|dict| dict.get(b"Rotate").ok().and_then(|value| value.as_i64().ok()))
                .unwrap_or(0);

            let rotation = (((current + angle) % 360) + 360) % 360;

            if let Ok(dict) = doc.get_dictionary_mut(page_id) {
                dict.set("Rotate", rotation);
            }
        }

        doc.save(output)?;
        Ok(count)
    }

}
