use sqlparser::dialect::GenericDialect;
use sqlparser::parser::{Parser, ParserError};
use std::backtrace::Backtrace;
use thiserror::Error;

#[derive(Debug, Error)]
enum WaterError {
    #[error("E1: Unknown")]
    Unknown,
    #[error("E2: Erroneous SQL statement")]
    ParseError {
        #[from]
        source: ParserError,
        backtrace: Backtrace,
    },
}

type WaterResult<T> = Result<T, WaterError>;

/// A Water ASM program compiled from SQL statements. Water VM can execute it.
struct WaterAsm {
    // todo
}

/// Compiles the sql into Water ASM.
fn compile(sql: &str) -> WaterResult<WaterAsm> {
    let dialect = GenericDialect {}; // or AnsiDialect, or your own dialect ...
    let ast = Parser::parse_sql(&dialect, sql)?;

    println!("{:#?}", ast);
    Ok(WaterAsm {})
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let sql = "SELECT * FROM Example";
        compile(sql).unwrap();
    }
}
