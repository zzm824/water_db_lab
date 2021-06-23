use sqlparser::dialect::GenericDialect;
use sqlparser::parser::{Parser, ParserError};
use std::backtrace::Backtrace;
use thiserror::Error;
use crate::{WaterResult};
use crate::vm::WaterAsm;
use sqlparser::ast::Statement;


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
