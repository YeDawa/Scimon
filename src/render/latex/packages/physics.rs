use std::collections::HashMap;

use crate::render::latex::{
    parser::Parser,
    tex_ast::LatexNode,
    packages::LatexPackage,
};

pub struct Physics;

/// Maximum number of braced arguments and whether an [optional] argument is
/// accepted, per command. Arguments are captured greedily up to the maximum
/// while a `{` follows, matching how these commands are written in practice.
fn arity(command: &str) -> (usize, bool) {
    match command {
        // derivatives: \dv[n]{f}{x}, \pdv[n]{f}{x}{y}
        "dv" | "fdv"          => (2, true),
        "pdv"                 => (3, true),
        // braket notation
        "bra" | "ket"         => (1, false),
        "braket" | "ip" | "dyad" | "ev" => (2, false),
        "mel"                 => (3, false),
        // commutators and Poisson brackets
        "comm" | "acomm" | "pb" => (2, false),
        // delimited quantities and vectors
        "abs" | "norm" | "order" | "va" | "vb" | "vu" => (1, false),
        _ => (1, false),
    }
}

impl LatexPackage for Physics {

    fn commands(&self) -> &'static [&'static str] {
        &[
            "dv", "pdv", "fdv",
            "bra", "ket", "braket", "ip", "dyad", "ev", "mel",
            "comm", "acomm", "pb",
            "abs", "norm", "order", "va", "vb", "vu",
        ]
    }

    /// physics-package commands in body text — reconstructed verbatim and
    /// wrapped into inline math, where MathJax's physics extension typesets
    /// them. Inside math mode the raw content reaches MathJax untouched.
    fn command(
        &self,
        command: &str,
        starred: bool,
        parser: &mut Parser,
        _labels: &mut HashMap<String, String>,
    ) -> Vec<LatexNode> {
        let (max_args, takes_optional) = arity(command);

        let mut tex = format!("\\{}{}", command, if starred { "*" } else { "" });
        if takes_optional {
            if let Some(option) = parser.parse_optional_arg() {
                tex.push_str(&format!("[{}]", option));
            }
        }

        let mut taken = 0;
        while taken < max_args {
            parser.lexer.skip_whitespace();
            if parser.peek() != Some('{') {
                break;
            }
            tex.push_str(&format!("{{{}}}", parser.parse_braces_content()));
            taken += 1;
        }

        vec![LatexNode::RawMathInline(tex)]
    }

}
