use serde::{Deserialize, Serialize};
use std::mem::size_of;

/// This enum enlists all the data types supported by Water DB.
///
/// The actual datum is stored type-erased. We use this enum to restore type
/// information.
#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub enum DataType {
    U32, // todo: in progress
}

impl DataType {
    pub fn len(&self) -> usize {
        match self {
            DataType::U32 => size_of::<u32>(),
            _ => unimplemented!(),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub enum Datum {
    U32(u32),
}
