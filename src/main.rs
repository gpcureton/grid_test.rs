use std::{collections::HashMap, env, error::Error, path::PathBuf};
// use log::{debug, error, info, trace, warn};
// use log;

// Looks for code in src/lib.rs
use data::config_logger;

// Looks for code in src/args.rs
pub mod args;

// Looks for code in src/reads.rs
pub mod inputs;
use inputs::reads as read_csv;
use read_csv::ReadRecord;

// Looks for code in src/compute.rs
pub mod compute;
use compute::HeightData;
use compute::MAX_RECORDS;

// Looks for code in src/writes.rs
pub mod outputs;
use outputs::{writes as write_csv, WriteRecord};

/// The main function
fn main() -> Result<(), Box<dyn Error>> {
    let sys_args: Vec<String> = env::args().collect();
    log::info!("Initial sys_args is: {sys_args:?}");
    let args = args::args(&sys_args)?;

    let verbosity: &u8 = args.get_one("verbosity").unwrap();
    let log_file: Option<&PathBuf> = args.get_one("log_file");

    log::info!("The verbosity is {verbosity}");
    log::info!("The input log filename is {log_file:?}");

    config_logger(verbosity, log_file)?;

    // log::error!("main() Goes to stderr and file");
    // log::warn!("main() Goes to stderr and file");
    // log::info!("main() Goes to stderr and file");
    // log::debug!("main() Goes to file only");
    // log::trace!("main() Goes to file only");

    let in_files: Vec<&PathBuf> = args.get_many("in_file").unwrap().collect();
    let out_file: &PathBuf = args.get_one("out_file").unwrap();
    let algorithm: &String = args.get_one("alg").unwrap();
    let grid_size: &f64 = args.get_one("grid_size").unwrap();

    log::info!("The alg is {algorithm}");

    let csv_records: Vec<ReadRecord> = match algorithm.as_str() {
        "serde" => {
            log::info!("We have chosen the serde branch.");
            read_csv::read_using_csv_serde(&in_files, &MAX_RECORDS)?
        }
        "csv" => {
            log::info!("We have chosen the csv branch.");
            read_csv::read_using_csv(&in_files, &MAX_RECORDS)?
        }
        _ => {
            log::info!("No branch was chosen.");
            Vec::new()
        }
    };

    let grid_dict: HashMap<String, HeightData> =
        compute::generate_histograms(&csv_records, grid_size)?;

    let output_records: Vec<WriteRecord> = compute::calc_stats(&grid_dict)?;

    write_csv::write_csv_using_serde(&output_records, out_file)?;

    Ok(())
}
