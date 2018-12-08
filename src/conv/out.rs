// https://qiita.com/algebroid/items/c456d4ec555ae04c7f92
use std::collections::HashMap;
use std::error::Error;
use std::io;

pub fn out(hdr: Vec<String>, data_map: HashMap<String, String>) -> Result<(), Box<Error>> {
    let mut wtr = csv::Writer::from_writer(io::stdout());

    wtr.write_record(&hdr)?;

    let mut row: Vec<String> = Vec::new();
    for hdr_col in hdr {
        if let Some(val) = data_map.get(&hdr_col) {
            row.push(val.to_string());
        }
    }

    wtr.write_record(&row)?;
    wtr.flush()?;

    Ok(())
}
