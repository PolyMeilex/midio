#![warn(future_incompatible)]
#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]

/// A MIDI structure used internally by some backends to store incoming
/// messages. Each message represents one and only one MIDI message.
/// The timestamp is represented as the elapsed microseconds since
/// a point in time that is arbitrary, but does not change for the
/// lifetime of a given MidiInputConnection.
#[allow(unused)]
#[derive(Debug, Clone)]
struct MidiMessage {
    bytes: Vec<u8>,
    timestamp: u64,
}

#[allow(unused)]
impl MidiMessage {
    fn new() -> MidiMessage {
        MidiMessage {
            bytes: vec![],
            timestamp: 0,
        }
    }
}

pub mod os; // include platform-specific behaviour

mod errors;
pub use errors::*;

mod common;
pub use common::*;

mod backend;
