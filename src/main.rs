use clap::{ Arg, App};
use serde_json;
use std::{ env, fs, io, path::{Path, PathBuf } };

fn main() {

    let matches = App::new("Actual Storage")
        .version("0.1")
        .author("Bryce Campbell <tonyhawk2100@gmail.com>")
        .about("Calculates the usable storage of a secondary storage devive with a specified capacity.")
        .arg(Arg::new("size")
        .about("the nurmerical capacity")
        .index(1)
        .required(true))
        .arg(Arg::new("unit")
        .about("unit of measure for storage capacity")
        .required(true)
        .takes_value(true)
        .index(2))
    .get_matches();

    let size: u32 = matches.value_of("size").unwrap().parse().unwrap();
    let unit: &str = matches.value_of("unit").unwrap();

    let calculation = calculate(size, unit);

    println!("Approximate capacity is: {:.2} {}", calculation, unit)
}

fn load_file(file: &PathBuf) -> Result<String, io::Error> {
    fs::read_to_string(file)
}

fn units() -> Vec<String> {

    let mut file_path = env::current_exe().unwrap();

    file_path = file_path.parent().unwrap().join("units.json");
    
    let content = match load_file(&file_path) {
        Ok(contents) => Some(contents),
        _ => None
    }.expect("Could not load file");

    let units: Vec<String> = serde_json::from_str(&content).expect("Could not decode file");

    return units;
}

fn calculate(size: u32, unit: &str) -> f64 {
    let index = units().iter().position(|u| u.to_lowercase() == unit.to_lowercase()).expect("unit not found");

    (f64::from(size)*1000_f64.powf(f64::from((index+1) as u32)))/1024_f64.powf(f64::from((index +1) as u32))
}
