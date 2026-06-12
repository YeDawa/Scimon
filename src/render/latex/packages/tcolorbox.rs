use std::collections::HashMap;

use crate::render::latex::{
    parser::Parser,
    tex_ast::LatexNode,
    packages::LatexPackage,
};

pub struct Tcolorbox;

impl LatexPackage for Tcolorbox {

    fn environments(&self) -> &'static [&'static str] {
        &["tcolorbox"]
    }

    fn environment(
        &self,
        env: &str,
        options: Option<String>,
        parser: &mut Parser,
        labels: &mut HashMap<String, String>,
    ) -> Vec<LatexNode> {
        let raw = parser.read_until_end(env);
        let inner = Parser::new(raw.trim()).parse(true, labels);
        let (title, colback, colframe) = parse_options(&options.unwrap_or_default());

        let mut nodes = vec![LatexNode::Text(format!(
            "<div class=\"latex-tcolorbox\" style=\"--tcb-back:{colback}; --tcb-frame:{colframe};\">",
            colback = colback, colframe = colframe,
        ))];
        if let Some(title) = title {
            nodes.push(LatexNode::Text(format!(
                "<div class=\"tcolorbox-title\" style=\"background:{};\">{}</div>", colframe, title
            )));
        }
        nodes.push(LatexNode::Text("<div class=\"tcolorbox-body\">".to_string()));
        nodes.extend(inner);
        nodes.push(LatexNode::Text("</div></div>".to_string()));

        nodes
    }

}

/// Parse tcolorbox key=value options, return (title, colback, colframe).
/// Commas inside {braced} values do not split, and outer braces are removed.
fn parse_options(opts: &str) -> (Option<String>, String, String) {
    let mut title:    Option<String> = None;
    let mut colback  = "#eaf4fb".to_string();
    let mut colframe = "#2980b9".to_string();

    let mut parts = Vec::new();
    let mut current = String::new();
    let mut depth = 0usize;
    for c in opts.chars() {
        match c {
            '{' => { depth += 1; current.push(c); }
            '}' => { depth = depth.saturating_sub(1); current.push(c); }
            ',' if depth == 0 => parts.push(std::mem::take(&mut current)),
            _ => current.push(c),
        }
    }
    parts.push(current);

    for part in parts {
        let Some((key, value)) = part.split_once('=') else { continue };
        let value = value.trim()
            .trim_start_matches('{')
            .trim_end_matches('}')
            .trim();
        match key.trim() {
            "title"    => title    = Some(value.to_string()),
            "colback"  => colback  = Parser::latex_color(value),
            "colframe" => colframe = Parser::latex_color(value),
            _ => {}
        }
    }
    (title, colback, colframe)
}
