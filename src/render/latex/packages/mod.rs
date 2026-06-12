use std::collections::HashMap;

use crate::render::latex::{
    parser::Parser,
    tex_ast::LatexNode,
};

pub mod mhchem;
pub mod physics;
pub mod siunitx;
pub mod tcolorbox;

/// A LaTeX package handled outside the core parser.
pub trait LatexPackage: Sync {

    /// Command words this package owns (without the backslash)
    fn commands(&self) -> &'static [&'static str] {
        &[]
    }

    /// Environment names this package owns
    fn environments(&self) -> &'static [&'static str] {
        &[]
    }

    /// Translate one of the declared commands into nodes. The command word
    /// arrives without the backslash; `starred` covers \cmd* variants.
    fn command(
        &self,
        _command: &str,
        _starred: bool,
        _parser: &mut Parser,
        _labels: &mut HashMap<String, String>,
    ) -> Vec<LatexNode> {
        Vec::new()
    }

    /// Translate one of the declared environments into nodes. `options` is
    /// the [...] group that followed \begin{env}, when present.
    fn environment(
        &self,
        _env: &str,
        _options: Option<String>,
        _parser: &mut Parser,
        _labels: &mut HashMap<String, String>,
    ) -> Vec<LatexNode> {
        Vec::new()
    }

}

/// Every registered package module, in lookup order — siunitx comes before
/// physics so \qty keeps its number-and-unit meaning.
static REGISTRY: &[&dyn LatexPackage] = &[
    &siunitx::Siunitx,
    &tcolorbox::Tcolorbox,
    &mhchem::Mhchem,
    &physics::Physics,
];

pub fn is_package_command(command: &str) -> bool {
    REGISTRY.iter().any(|package| package.commands().contains(&command))
}

pub fn is_package_environment(env: &str) -> bool {
    REGISTRY.iter().any(|package| package.environments().contains(&env))
}

pub fn command(
    command: &str,
    starred: bool,
    parser: &mut Parser,
    labels: &mut HashMap<String, String>,
) -> Vec<LatexNode> {
    for package in REGISTRY {
        if package.commands().contains(&command) {
            return package.command(command, starred, parser, labels);
        }
    }
    Vec::new()
}

pub fn environment(
    env: &str,
    options: Option<String>,
    parser: &mut Parser,
    labels: &mut HashMap<String, String>,
) -> Vec<LatexNode> {
    for package in REGISTRY {
        if package.environments().contains(&env) {
            return package.environment(env, options, parser, labels);
        }
    }
    Vec::new()
}
