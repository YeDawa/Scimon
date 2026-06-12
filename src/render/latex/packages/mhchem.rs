use std::collections::HashMap;

use crate::render::latex::{
    parser::Parser,
    tex_ast::LatexNode,
    packages::LatexPackage,
};

pub struct Mhchem;

impl LatexPackage for Mhchem {

    fn commands(&self) -> &'static [&'static str] {
        &["ce", "pu"]
    }

    // \ce{H2O} / \pu{123 kJ.mol-1} in body text — inside math mode the raw
    // content already reaches MathJax, whose mhchem extension typesets it;
    // here the command just needs to be wrapped into inline math.
    fn command(
        &self,
        command: &str,
        _starred: bool,
        parser: &mut Parser,
        _labels: &mut HashMap<String, String>,
    ) -> Vec<LatexNode> {
        let formula = parser.parse_braces_content();
        vec![LatexNode::RawMathInline(format!("\\{}{{{}}}", command, formula))]
    }

}
