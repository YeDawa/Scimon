use std::collections::HashMap;

use crate::render::latex::{
    tex_ast::LatexNode,
    context::RenderContext,
};

pub struct Misc;

impl Misc {

    pub fn ctx_eq_num_peek(labels: &HashMap<String, String>) -> usize {
        labels.values()
            .filter_map(|v| v.parse::<usize>().ok())
            .max()
            .unwrap_or(0)
    }

    pub fn register_inner_labels(nodes: &[LatexNode], value: &str, ctx: &mut RenderContext) {
        for node in nodes {
            if let LatexNode::Label(name) = node {
                ctx.labels.insert(name.clone(), value.to_string());
            }
        }
    }

    pub fn extract_and_register_labels(
        raw: &str,
        value: &str,
        prefix: &str,
        labels: &mut HashMap<String, String>,
    ) {
        let tag = "\\label{";
        let mut pos = 0;
        while pos + tag.len() <= raw.len() {
            if raw[pos..].starts_with(tag) {
                pos += tag.len();
                let start = pos;
                while pos < raw.len() && raw.as_bytes()[pos] != b'}' {
                    pos += 1;
                }

                let key = &raw[start..pos];
                if prefix.is_empty() || key.starts_with(prefix) {
                    labels.insert(key.to_string(), value.to_string());
                }

            } else {
                pos += 1;
            }
        }
    }

}