use serde_json;
use std::{ fs, io, path::Path};

fn main() {
    let unit = "GB";

    let calculation = calculate(250, unit);

    println!("Approximate capacity is: {:.2} {}", calculation, unit)
}

fn load_file(file: &Path) -> Result<String, io::Error> {
    fs::read_to_string(file)
}

fn units() -> Vec<String> {
    let file_path = Path::new("units.json");
    
    let content = match load_file(&file_path) {
        Ok(contents) => Some(contents),
        _ => None
    }.expect("Could not load file");

    let units: Vec<String> = serde_json::from_str(&content).expect("Could not decode file");

    return units;
}

fn calculate(size: u32, unit: &str) -> f64 {
    let index = units().iter().position(|u| u == unit).expect("unit not found");

    (f64::from(size)*1000_f64.powf(f64::from((index+1) as u32)))/1024_f64.powf(f64::from((index +1) as u32))
}
