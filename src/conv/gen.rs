use std::collections::HashMap;
use std::error::Error;
use std::ffi::OsString;
//use csv::ReaderBuilder;

pub fn gen(data_csv_path: OsString, cfg: HashMap<(usize, usize), String>) -> Result<HashMap<String, String>, Box<Error>> {
    // https://docs.rs/csv/1.0.0-beta.1/csv/struct.ReaderBuilder.html
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_path(data_csv_path)?;

    let mut data_map: HashMap<String, String> = HashMap::new();

    for (i, result) in rdr.records().enumerate() {
        let record = result?;
        let row_index = i + 1;

        for (j, val) in record.iter().enumerate() {
            let col_index = j + 1;
            let cfg_key = (row_index, col_index);
            
            if let Some(header_column_name) = cfg.get(&cfg_key) {
                data_map.insert(
                    header_column_name.to_string(),
                    val.to_string(),
                    );
            }
        }
    }

    Ok(data_map)
}


