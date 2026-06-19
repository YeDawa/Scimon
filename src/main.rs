// `Lazy` is stored in associated `const`s (can't be `static` inside `impl`);
// the interior-mutability lints are benign for this read-only-after-init use.
#![allow(clippy::declare_interior_mutable_const)]
#![allow(clippy::borrow_interior_mutable_const)]
// Pre-existing structural/style choices in the LaTeX & render modules.
#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::if_same_then_else)]
#![allow(clippy::type_complexity)]

mod ui;
mod cmd;
mod utils;
mod regexp;
mod consts;
mod addons;
mod system;
mod syntax;
mod scimon;
mod render;
mod monlib;
mod server;
mod helpers;
mod configs;
mod security;
mod args_cli;
mod handlers;
mod templates;
mod generator;

use crate::scimon::Scimon;

#[tokio::main]
async fn main() {
    Scimon.init().await;
}
