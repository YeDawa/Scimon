use crate::render::latex::{
    tex_ast::LatexNode,
    context::RenderContext,
};

pub struct Nodes;

impl Nodes {

    pub fn render(nodes: &[LatexNode], ctx: &mut RenderContext) -> String {
        nodes.iter().map(|node| node.to_html(ctx)).collect()
    }

}