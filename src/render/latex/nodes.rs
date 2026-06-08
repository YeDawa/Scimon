use crate::render::latex::{
    tex_ast::LatexNode,
    context::RenderContext,
};

pub struct Nodes;

impl Nodes {

    pub fn render(nodes: &[LatexNode], ctx: &mut RenderContext) -> String {
        nodes.iter().map(|node| node.to_html(ctx)).collect()
    }

    pub fn pre_pass(nodes: &[LatexNode], ctx: &mut RenderContext) {
        for node in nodes {
            match node {
                LatexNode::Section(title) => {
                    ctx.sec_num += 1; ctx.subsec_num = 0;
                    ctx.last_counter = format!("{}", ctx.sec_num);
                    ctx.toc.push((1, ctx.last_counter.clone(), title.clone()));
                }

                LatexNode::Subsection(title) => {
                    ctx.subsec_num += 1;
                    ctx.last_counter = format!("{}.{}", ctx.sec_num, ctx.subsec_num);
                    ctx.toc.push((2, ctx.last_counter.clone(), title.clone()));
                }
                
                LatexNode::EquationBlock(inner) => {
                    ctx.eq_num += 1; 
                    ctx.last_counter = format!("{}", ctx.eq_num);

                    Nodes::pre_pass(inner, ctx); 
                }

                LatexNode::Image(_) => {
                    ctx.fig_num += 1; ctx.last_counter = format!("{}", ctx.fig_num);
                }
                
                LatexNode::Table(rows) => {
                    ctx.tab_num += 1; 
                    ctx.last_counter = format!("{}", ctx.tab_num);
                    for row in rows { 
                        for cell in row { Nodes::pre_pass(cell, ctx); } 
                    }
                }
                
                LatexNode::Label(key) => {
                    ctx.labels.insert(key.clone(), ctx.last_counter.clone());
                }
                
                LatexNode::Bold(inner) | LatexNode::Italic(inner) | LatexNode::MathInline(inner) => {
                    Nodes::pre_pass(inner, ctx);
                }

                LatexNode::Itemize(items) | LatexNode::Enumerate(items) => {
                    for item in items { Nodes::pre_pass(item, ctx); }
                }
                
                _ => {}
            }
        }
    }

}