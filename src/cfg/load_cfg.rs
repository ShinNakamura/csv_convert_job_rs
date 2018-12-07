// extern crate ... はクレートのルートで宣言する必要がある
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;

use std::collections::HashMap;
use std::error::Error;
use std::ffi::OsString;

#[derive(Debug, Deserialize)]
struct Record {
    row: usize,
    column: usize,
    name: String,
}

pub fn load(config_csv_path: OsString) -> Result<HashMap<(usize, usize), String>, Box<Error>> {
    let mut rdr = csv::Reader::from_path(config_csv_path)?;
    let mut cfg = HashMap::new();
    for result in rdr.deserialize() {
        let record: Record = result?;
        cfg.insert(
            (record.row, record.column,),
            record.name
            );
    }

    Ok(cfg)
}


