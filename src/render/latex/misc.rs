use crate::render::latex::{
    tex_ast::LatexNode,
    context::RenderContext,
};

pub struct Misc;

impl Misc {

    pub fn register_inner_labels(nodes: &[LatexNode], value: &str, ctx: &mut RenderContext) {
        for node in nodes {
            if let LatexNode::Label(name) = node {
                ctx.labels.insert(name.clone(), value.to_string());
            }
        }
    }

}