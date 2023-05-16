use std::{
    error::Error,
    collections::HashMap,
    process
};

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
use outputs::writes as write_csv;
// use writes::writes as write_csv;
// use write_csv::WriteRecord as WriteRecord;
// use crate::writes::writes::WriteRecord as WriteRecord;

/// The main function
fn main() -> Result<(), Box<dyn Error>> {

    // let _args = args::args();

    // let file_path = "./data/viirs_cloud_top_height_full.csv".to_string();
    let file_path = "./data/viirs_cloud_top_height.csv".to_string();

    // let result = read_using_include_str()?;
    // let result = read_using_csv()?;
    //
    let csv_records: Vec<ReadRecord> = read_csv::read_using_csv_serde(&file_path, &MAX_RECORDS)?;

    let grid_dict: HashMap<String, HeightData> = compute::generate_histograms(&csv_records)?;

    let output_records = compute::calc_stats(&grid_dict)?;

    write_csv::write_csv_using_serde(&output_records)?;

    process::exit(0)
}
