extern crate csv_convert_job_rs;
use csv_convert_job_rs::arg::get_nth_arg;
use csv_convert_job_rs::cfg::load_cfg;
use csv_convert_job_rs::conv::gen;
use csv_convert_job_rs::conv::out;

use std::error::Error;
use std::process;

fn run() -> Result<(), Box<Error>> {
    let config_csv_path = get_nth_arg::get(1)?;
    // cfg は辞書。hdrはconfig.csvの登録順を基準に出力時のヘッダーを定義したもの
    let (cfg, hdr) = load_cfg::load(config_csv_path)?;
    //println!("{:#?}", cfg);
    //println!("{:#?}", hdr);

    let data_csv_path = get_nth_arg::get(2)?;
    //println!("{:#?}", data_csv_path);

    // data.csv + cfg = data_map
    let data_map = gen::gen(data_csv_path, cfg)?;
    //println!("{:#?}", data_map);

    // hdr + data_map = output(csv)
    let _ = out::out(hdr, data_map)?;

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
