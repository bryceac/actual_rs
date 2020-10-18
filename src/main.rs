use clap::{ Arg, App}; // make CLI app easier to create by importing clap.
use serde_json; // import serde_json library to read JSON.

#[cfg(debug_assertions)]
use std::{ fs, io, path::PathBuf }; // import stuff needed to read files

#[cfg(not(debug_assertions))]
use std::{ env, fs, io, path::PathBuf }; // import stuff need to read files.

fn main() {

    // setup application and specify arguments, with help text.
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

    // retrieve numerical size from input
    let size: u32 = matches.value_of("size").unwrap().parse().expect("Value must be a positive number");

    let unit: &str = matches.value_of("unit").expect("Could not retrieve unit.");

    // preform calculation
    let calculation = calculate(size, unit);

    // output results rounded to 2 decimal places.
    println!("Approximate capacity is: {:.2} {}", calculation, unit)
}

// load file at a given path.
fn load_file(file: &PathBuf) -> Result<String, io::Error> {
    fs::read_to_string(file)
}

// load units from json file
#[cfg(not(debug_assertions))]
fn units() -> Vec<String> {

    // retrieve executable's path, so that program works properly after installation on Windows. 
    let mut file_path = env::current_exe().unwrap();

    // get the executable's directory
    file_path = file_path.parent().unwrap().join("units.json");
    
    // try to retrieve file content
    let content = match load_file(&file_path) {
        Ok(contents) => Some(contents),
        _ => None
    }.expect("Could not load file");

    // parse file content as a String vector
    let units: Vec<String> = serde_json::from_str(&content).expect("Could not decode file");

    // result units found in JSON
    return units;
}

// load units from json file for debugging
#[cfg(debug_assertions)]
fn units() -> Vec<String> {

    // path for json
    let mut file_path = PathBuf::new();
    file_path.push("units.json");
    
    // try to retrieve file content
    let content = match load_file(&file_path) {
        Ok(contents) => Some(contents),
        _ => None
    }.expect("Could not load file");

    // parse file content as a String vector
    let units: Vec<String> = serde_json::from_str(&content).expect("Could not decode file");

    // result units found in JSON
    return units;
}

// perform calculation based on specified size and unit
fn calculate(size: u32, unit: &str) -> f64 {

    // attempt to retrieve index of specified unit
    let index: u32 = units().iter().position(|u| u.to_lowercase() == unit.to_lowercase()).expect("unit not found") as u32;

    // return result of calculation of formula (s*1000^x)/1024^x, where x is 1 high that the index of the specified unit.
    f64::from(size)*1000_f64.powf(f64::from(index+1))/1024_f64.powf(f64::from(index +1))
}
