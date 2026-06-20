use std::{
    fs,
    error::Error,
};

use epub_builder::{
    ZipLibrary,
    EpubBuilder,
    EpubContent,
    ReferenceType,
};

use crate::system::markdown::Markdown;

pub struct Epub;

impl Epub {

    fn xml_escape(value: &str) -> String {
        value
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
    }

    // Packages a Markdown string as a single-chapter EPUB.
    pub fn create(&self, markdown: &str, title: &str, author: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
        let html_body = Markdown.append_extras_and_render(markdown);

        let xhtml = format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
             <!DOCTYPE html>\n\
             <html xmlns=\"http://www.w3.org/1999/xhtml\"><head><title>{}</title></head>\
             <body>{}</body></html>",
            Self::xml_escape(title), html_body
        );

        let mut output = Vec::new();
        let mut builder = EpubBuilder::new(ZipLibrary::new()?)?;

        builder.metadata("title", title)?;
        builder.metadata("author", author)?;
        builder.add_content(
            EpubContent::new("chapter1.xhtml", xhtml.as_bytes())
                .title(title)
                .reftype(ReferenceType::Text),
        )?;

        builder.generate(&mut output)?;
        fs::write(output_path, output)?;
        Ok(())
    }

}
