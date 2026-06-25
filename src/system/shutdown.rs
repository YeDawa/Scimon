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

// Process-wide cancellation flag, flipped by Ctrl+C and polled by every
// long-running step so a run can stop gracefully instead of being killed.
static CANCELLED: AtomicBool = AtomicBool::new(false);

pub struct Shutdown;

impl Shutdown {

    // Registers the single Ctrl+C handler for the process. The first press
    // requests a graceful stop (steps finish the current item and bail); a
    // second press forces an immediate exit.
    pub fn init(&self) {
        let _ = ctrlc::set_handler(|| {
            if CANCELLED.swap(true, Ordering::SeqCst) {
                process::exit(130);
            }

            let now = General.date_time();
            println!(
                "\n{} Interrupting… press Ctrl+C again to force quit.",
                now.yellow().bold(),
            );
        });
    }

    // True once a graceful shutdown has been requested.
    pub fn cancelled(&self) -> bool {
        CANCELLED.load(Ordering::SeqCst)
    }

}
