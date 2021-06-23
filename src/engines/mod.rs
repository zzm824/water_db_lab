//! Storage engines.
//!
//! Water Storage Engines (hereafter *engines*) are the backend of Water DB.
//! They manage the actual storage media and expose a consistent interface
//! using iterators (*cursors* in database jargon).

use crate::data_types::Datum;

mod water_sled_engine;

/// A cursor iterates in a table.
pub trait ForwardCursor {
    fn next(&mut self) -> Datum;
}

pub trait WaterEngine {
    // todo
    fn forward_cursor(&self) -> Box<dyn ForwardCursor>;
}