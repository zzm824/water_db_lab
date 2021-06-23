use crate::data_types::DataType;
use crate::WaterResult;
use serde::{Deserialize, Serialize};
use sled::Db;

/// A Water Storage Engine using *Sled* in implementation.
pub struct WaterSledEngine {
    sled: Db,
}

impl WaterSledEngine {
    pub fn open(filename: &str) -> WaterResult<Self> {
        let db = sled::open(filename)?;

        todo!();

        Ok(Self { sled: db })
    }
}

/// In-memory, column-oriented table.
#[derive(Serialize, Deserialize)]
struct Table {
    name: String,
    column_types: Vec<DataType>,
    data: Vec<Vec<u8>>,
}

impl Table {
    pub fn num_columns(&self) -> usize {
        self.column_types.len()
    }

    pub fn num_rows(&self) -> usize {
        if self.num_columns() == 0 {
            0
        } else {
            self.data[0].len() / self.column_types[0].len()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_table_generation() {
        let t = Table {
            name: "Example".into(),
            column_types: vec![DataType::U32, DataType::U32],
            data: vec![123u32.to_ne_bytes().to_vec(), 432u32.to_ne_bytes().to_vec()],
        };

        assert_eq!(
            serde_json::to_string(&t).unwrap(),
            r#"{"name":"Example","column_types":["U32","U32"],"data":[[123,0,0,0],[176,1,0,0]]}"#
        );
    }
}
