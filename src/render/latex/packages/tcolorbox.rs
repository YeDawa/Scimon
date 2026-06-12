//! tcolorbox — \begin{tcolorbox}[title=..., colback=..., colframe=...]

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
fn parse_options(opts: &str) -> (Option<String>, String, String) {
    let mut title:    Option<String> = None;
    let mut colback  = "#eaf4fb".to_string();
    let mut colframe = "#2980b9".to_string();

    for part in opts.split(',') {
        let kv: Vec<&str> = part.splitn(2, '=').collect();
        if kv.len() != 2 { continue; }
        match kv[0].trim() {
            "title"    => title    = Some(kv[1].trim().to_string()),
            "colback"  => colback  = Parser::latex_color(kv[1].trim()),
            "colframe" => colframe = Parser::latex_color(kv[1].trim()),
            _ => {}
        }
    }
    (title, colback, colframe)
}
