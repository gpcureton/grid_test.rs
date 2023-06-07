use std::println;
use std::{
    error::Error,
    collections::HashMap,
};

// Looks for code in src/args.rs
pub mod args;

// Looks for code in src/reads.rs
pub mod inputs;
use inputs::reads as read_csv;
use read_csv::ReadRecord as ReadRecord;

// Looks for code in src/compute.rs
pub mod compute;
use compute::{MAX_RECORDS, GRID_SIZE};
use compute::HeightData as HeightData;

// Looks for code in src/writes.rs
pub mod outputs;
use outputs::{writes as write_csv, WriteRecord};
// use writes::writes as write_csv;
// use write_csv::WriteRecord as WriteRecord;
// use crate::writes::writes::WriteRecord as WriteRecord;

/// The main function
fn main() -> Result<(), Box<dyn Error>> {

    let args = args::args()?;

    let in_files: Vec<&String> = args.get_many("in_file").unwrap().collect();
    println!("{in_files:?}");
    let out_file: &String = args.get_one("out_file").unwrap();
    println!("{out_file:?}");

    // // TODO: This should be a command line option
    // // let result = read_using_include_str()?;
    // // let result = read_using_csv()?;
    let csv_records: Vec<ReadRecord> = read_csv::read_using_csv_serde(&in_files, &MAX_RECORDS)?;

    let grid_dict: HashMap<String, HeightData> = compute::generate_histograms(&csv_records)?;

    let output_records: Vec<WriteRecord> = compute::calc_stats(&grid_dict)?;

    write_csv::write_csv_using_serde(&output_records, out_file)?;

    Ok(())
}
