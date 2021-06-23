//! Storage engines.
//!
//! Water Storage Engines (hereafter *engines*) are the backend of Water DB.
//! They manage the actual storage media and expose a consistent interface
//! using iterators (*cursors* in database jargon).

mod water_sled_engine;


pub trait WaterEngine {
    // todo
}