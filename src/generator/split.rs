use lopdf::Document;
use std::error::Error;

use crate::{
    syntax::vars::Vars,

    ui::{
        ui_base::UI,
        errors_alerts::ErrorsAlerts,
        success_alerts::SuccessAlerts,
    },
};

pub struct Split {
    contents: String,
}

impl Split {

    pub fn new(contents: &str) -> Self {
        Self {
            contents: contents.to_string(),
        }
    }

    pub fn get(&self) {
        let jobs = Vars.get_all_split(&self.contents);

        if jobs.is_empty() {
            return;
        }

        UI::section_header("Splitting PDFs", "normal");

        for (input, template) in jobs {
            match self.split_one(&input, &template) {
                Ok(count) => SuccessAlerts::split(&input, count),
                Err(e) => ErrorsAlerts::generic(&e.to_string()),
            }
        }
    }

    fn split_one(&self, input: &str, template: &str) -> Result<usize, Box<dyn Error>> {
        let base = Document::load(input)?;
        let pages: Vec<u32> = base.get_pages().keys().copied().collect();

        if pages.is_empty() {
            return Err(format!("No pages found in: {}", input).into());
        }

        let mut count = 0;

        // Keeping one page is done by cloning the document and deleting all the
        // others, so each output keeps that page's resources intact.
        for &page in &pages {
            let mut doc = base.clone();

            let others: Vec<u32> = pages.iter().copied().filter(|&n| n != page).collect();
            doc.delete_pages(&others);
            doc.prune_objects();
            doc.compress();

            doc.save(Self::output_for(template, page))?;
            count += 1;
        }

        Ok(count)
    }

    // Builds a page's output path: `{n}` is replaced with the page number, or
    // the number is inserted before the extension when no placeholder is given.
    fn output_for(template: &str, page: u32) -> String {
        if template.contains("{n}") {
            return template.replace("{n}", &page.to_string());
        }

        match template.rfind('.') {
            Some(dot) => format!("{}-{}{}", &template[..dot], page, &template[dot..]),
            None => format!("{}-{}", template, page),
        }
    }

}
