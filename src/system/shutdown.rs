extern crate colored;

use colored::*;

use std::{
    process,

    sync::atomic::{
        Ordering,
        AtomicBool,
    },
};

use crate::system::general::General;

static CANCELLED: AtomicBool = AtomicBool::new(false);

pub struct Shutdown;

impl Shutdown {

    pub fn init(&self) {
        let _ = ctrlc::set_handler(|| {
            if CANCELLED.swap(true, Ordering::SeqCst) {
                process::exit(0);
            }

            let now = General.date_time();
            println!(
                "\n{} Interrupting… press Ctrl+C again to force quit.",
                now.yellow().bold(),
            );
        });
    }

    pub fn cancelled(&self) -> bool {
        CANCELLED.load(Ordering::SeqCst)
    }

}
