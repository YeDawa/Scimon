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
mod helpers;
mod configs;
mod security;
mod args_cli;
mod handlers;
mod generator;

use crate::scimon::Scimon;

#[tokio::main]
async fn main() {
    Scimon.init().await;
}
