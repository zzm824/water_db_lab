#![allow(unused_imports)]
#![allow(dead_code)]
#![feature(backtrace)]

use sqlparser::parser::ParserError;
use std::backtrace::Backtrace;
use thiserror::Error;

mod compiler;
mod data_types;
mod engines;
mod vm;

/// A handle to a Water DB instance.
pub struct WaterDb {}

#[derive(Debug, Error)]
pub enum WaterError {
    #[error("E1: Unknown")]
    Unknown,
    #[error("E2: Erroneous SQL statement")]
    ParseError {
        #[from]
        source: ParserError,
        backtrace: Backtrace,
    },
    #[error("E3: Backend error (specific to *sled* backend)")]
    SledError {
        #[from]
        source: sled::Error,
        backtrace: Backtrace,
    },
}

pub type WaterResult<T> = Result<T, WaterError>;


#[cfg(test)]
mod test {
    #[test]
    fn main_test() {}
}
