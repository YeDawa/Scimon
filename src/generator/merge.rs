use glob::glob;
use std::collections::BTreeMap;

use std::{
    error::Error,
    path::PathBuf,
};

use lopdf::{
    Object,
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

pub struct Merge {
    contents: String,
}

impl Merge {

    pub fn new(contents: &str) -> Self {
        Self {
            contents: contents.to_string(),
        }
    }

    pub fn get(&self) {
        let jobs = Vars.get_all_merge(&self.contents);

        if jobs.is_empty() {
            return;
        }

        UI::section_header("Merging PDFs", "normal");
        for (pattern, output) in jobs {
            match self.merge_one(&pattern, &output) {
                Ok(count) => SuccessAlerts::merged(&output, count),
                Err(e) => ErrorsAlerts::generic(&e.to_string()),
            }
        }
    }

    fn merge_one(&self, source: &str, output: &str) -> Result<usize, Box<dyn Error>> {
        let paths = self.inputs(source);

        if paths.is_empty() {
            return Err(format!("No files matched: {}", source).into());
        }

        let documents: Vec<Document> = paths.iter()
            .filter_map(|path| Document::load(path).ok())
            .collect();

        if documents.is_empty() {
            return Err(format!("No readable PDFs matched: {}", source).into());
        }

        let count = documents.len();
        let mut merged = Self::merge_documents(documents)
            .ok_or("Failed to merge PDFs: missing catalog or pages root.")?;

        merged.save(output)?;
        Ok(count)
    }

    // Resolves the input list: a `[ "a.pdf", "b.pdf" ]` list keeps its order,
    // while a `"glob"` pattern is expanded and sorted alphabetically.
    fn inputs(&self, source: &str) -> Vec<PathBuf> {
        if let Some(inner) = source.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
            return self.split_items(inner).into_iter().map(PathBuf::from).collect();
        }

        let mut paths: Vec<PathBuf> = glob(source)
            .map(|paths| paths.filter_map(Result::ok).collect())
            .unwrap_or_default();

        paths.sort();
        paths
    }

    fn split_items(&self, raw: &str) -> Vec<String> {
        raw.split(',')
            .map(|item| item.trim().trim_matches(|c| c == '"' || c == '\'').to_string())
            .filter(|item| !item.is_empty())
            .collect()
    }

    fn merge_documents(documents: Vec<Document>) -> Option<Document> {
        let mut max_id = 1;
        let mut documents_pages = BTreeMap::new();
        let mut documents_objects = BTreeMap::new();
        let mut document = Document::with_version("1.5");

        for mut doc in documents {
            doc.renumber_objects_with(max_id);
            max_id = doc.max_id + 1;

            documents_pages.extend(
                doc.get_pages()
                    .into_values()
                    .map(|object_id| (object_id, doc.get_object(object_id).unwrap().to_owned()))
                    .collect::<BTreeMap<ObjectId, Object>>(),
            );

            documents_objects.extend(doc.objects);
        }

        let mut catalog_object: Option<(ObjectId, Object)> = None;
        let mut pages_object: Option<(ObjectId, Object)> = None;

        for (object_id, object) in documents_objects.iter() {
            match object.type_name().unwrap_or("") {
                "Catalog" => {
                    catalog_object = Some((
                        catalog_object.as_ref().map(|(id, _)| *id).unwrap_or(*object_id),
                        object.clone(),
                    ));
                }

                "Pages" => {
                    if let Ok(dictionary) = object.as_dict() {
                        let mut dictionary = dictionary.clone();

                        if let Some((_, ref previous)) = pages_object {
                            if let Ok(old_dictionary) = previous.as_dict() {
                                dictionary.extend(old_dictionary);
                            }
                        }

                        pages_object = Some((
                            pages_object.as_ref().map(|(id, _)| *id).unwrap_or(*object_id),
                            Object::Dictionary(dictionary),
                        ));
                    }
                }

                "Page" | "Outlines" | "Outline" => {}

                _ => {
                    document.objects.insert(*object_id, object.clone());
                }
            }
        }

        let pages_object = pages_object?;
        let catalog_object = catalog_object?;

        for (object_id, object) in documents_pages.iter() {
            if let Ok(dictionary) = object.as_dict() {
                let mut dictionary = dictionary.clone();
                dictionary.set("Parent", pages_object.0);
                document.objects.insert(*object_id, Object::Dictionary(dictionary));
            }
        }

        if let Ok(dictionary) = pages_object.1.as_dict() {
            let mut dictionary = dictionary.clone();
            dictionary.set("Count", documents_pages.len() as u32);
            dictionary.set(
                "Kids",
                documents_pages.into_iter()
                    .map(|(object_id, _)| Object::Reference(object_id))
                    .collect::<Vec<_>>(),
            );

            document.objects.insert(pages_object.0, Object::Dictionary(dictionary));
        }

        if let Ok(dictionary) = catalog_object.1.as_dict() {
            let mut dictionary = dictionary.clone();
            dictionary.set("Pages", pages_object.0);
            dictionary.remove(b"Outlines");
            document.objects.insert(catalog_object.0, Object::Dictionary(dictionary));
        }

        document.trailer.set("Root", catalog_object.0);
        document.max_id = document.objects.len() as u32;
        document.renumber_objects();
        document.compress();

        Some(document)
    }

}
