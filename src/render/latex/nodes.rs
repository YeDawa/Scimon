use crate::render::latex::{
    tex_ast::LatexNode,
    context::RenderContext,
};

pub struct Nodes;

impl Nodes {

    /// Append HTML for every node in `nodes` directly into `buf`.
    /// Prefer this over `render` to avoid extra allocations in hot paths.
    pub fn write(nodes: &[LatexNode], ctx: &mut RenderContext, buf: &mut String) {
        for node in nodes {
            node.write_html(ctx, buf);
        }
    }

    /// Render `nodes` to a new `String`.
    pub fn render(nodes: &[LatexNode], ctx: &mut RenderContext) -> String {
        let mut buf = String::with_capacity(nodes.len() * 32);
        Self::write(nodes, ctx, &mut buf);
        buf
    }

}