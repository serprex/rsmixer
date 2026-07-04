mod callbacks;
pub mod common;
mod errors;
mod monitor;
mod pa_actions;
mod pa_interface;

use common::*;
pub use pa_interface::start;

#[derive(Debug)]
pub enum PAInternal {
    Tick,
    Command(Box<PulseAudioAction>),
    AskInfo(EntryIdentifier),
}
