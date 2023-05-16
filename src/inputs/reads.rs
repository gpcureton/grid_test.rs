use std::error::Error;
use serde::Deserialize;

use crate::{MAX_RECORDS, GRID_SIZE};

/// The Record struct holds a single line of data read from a csv file
#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "PascalCase")]
pub struct ReadRecord {
    pub longitude: f64,
    pub latitude: f64,
    pub height: i32,
}

/// Reads in a CSV file, using the csv crate and deserializing with serde crate.
/// Returns Ok(Vec<ReadRecord>).
pub fn read_using_csv_serde(file_path: &String, max_records: &usize) -> Result<Vec<ReadRecord>, Box<dyn Error>> {
    println!("Reading the file using csv crate with serde deserialization...");
    println!("GRID_SIZE = {GRID_SIZE}");


    let mut num_records: i32 = 0;
    // let mut csv_records: Vec<ReadRecord> = Vec::new();
    let mut csv_records: Vec<ReadRecord> = Vec::with_capacity(MAX_RECORDS);

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(file_path)?;

    for result in rdr.deserialize() {
        let record: ReadRecord = result?;

        // if num_obs as usize > MAX_RECORDS {
        if num_records as usize > *max_records {
            println!("Breaking at {}", num_records);
            break;
        }
        num_records += 1;

        let longitude = record.longitude;
        let latitude = record.latitude;
        let height = record.height;

        // When the struct field names are the same as the variables they are being populated with,
        // we can replace '"fieldname": varname' with just "varname".
        let record = ReadRecord {
            longitude,
            latitude,
            height,
        };
        csv_records.push(record);
    }

    println!("Finished deserializing the csv file...");
    println!("There are {:?} entries in the csv file.\n", num_records);

    Ok(csv_records)
}

/// Reads in a CSV file using the include_str macro, returning a Result containing a Vector of
/// Record structs.
/// Returns Ok(Vec<ReadRecord>).
pub fn read_using_include_str() -> Result<Vec<ReadRecord>, Box<dyn Error>> {
    println!("Reading the file using include_str macro...");

    let mut num_obs: i32 = 0;
    // let mut csv_records: Vec<ReadRecord> = Vec::new();
    let mut csv_records: Vec<ReadRecord> = Vec::with_capacity(MAX_RECORDS);

    let puzzle =
        include_str!("/home/geoffc/Computer_Stuff/Python/ChatGPT/viirs_cloud_top_height.csv")
            .lines();

    for (idx, line) in puzzle.enumerate() {
        if line.is_empty() || idx > MAX_RECORDS {
            break;
        }
        num_obs += 1;

        let v: Vec<&str> = line.split(',').collect();
        let longitude = v[0].parse::<f64>()?;
        let latitude = v[1].parse::<f64>()?;
        let height = v[2].parse::<i32>()?;

        // When the struct field names are the same as the variables they are being populated with,
        // we can replace '"fieldname": varname' with just "varname".
        let record = ReadRecord {
            longitude,
            latitude,
            height,
        };
        csv_records.push(record);
    }

    println!("Finished looping through the lines...");
    println!("There are {:?} observations", num_obs);

    Ok(csv_records)
}

/// Reads in a CSV file, using the csv crate and manually deserializing.
/// Returns Ok(Vec<ReadRecord>).
pub fn read_using_csv() -> Result<Vec<ReadRecord>, Box<dyn Error>> {
    println!("Reading the file using csv crate with manual destructuring...");

    let mut num_obs: i32 = 0;
    // let mut csv_records: Vec<ReadRecord> = Vec::new();
    let mut csv_records: Vec<ReadRecord> = Vec::with_capacity(MAX_RECORDS);

    let file_path = "./data/viirs_cloud_top_height.csv";

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(file_path)?;

    for result in rdr.records() {
        let record = result?;

        if num_obs as usize > MAX_RECORDS {
            break;
        }
        num_obs += 1;

        let longitude: f64 = record[0].parse()?;
        let latitude: f64 = record[1].parse()?;
        let height: i32 = record[2].parse()?;

        // When the struct field names are the same as the variables they are being populated with,
        // we can replace '"fieldname": varname' with just "varname".
        let record = ReadRecord {
            longitude,
            latitude,
            height,
        };
        csv_records.push(record);
    }

    println!("Finished looping through the lines...");
    println!("There are {:?} observations", num_obs);

    Ok(csv_records)
}
