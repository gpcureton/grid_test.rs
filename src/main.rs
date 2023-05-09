use std::{
    error::Error,
};
use std::collections::HashMap;
// use csv;
// use anyhow::Result;

// This lets us write `#[derive(Deserialize)]`.
use serde::{Serialize, Deserialize};

const GRID_SIZE: f64 = 1.0;
// const MAX_RECORDS: usize = 500;
const MAX_RECORDS: usize = 10_000_000;
// const MAX_RECORDS: usize = std::usize::MAX;

/// The Record struct holds a single line of data read from a csv file
///
/// Notice that the field names in this struct are NOT in the same order as
/// the fields in the CSV data!
#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "PascalCase")]
struct ReadRecord {
    longitude: f64,
    latitude: f64,
    height: i32,
}

/// The HeightData struct holds binned observations of a gridcell
/// Also stored is the number of observations for that gridcell
#[derive(Clone, Debug)]
struct HeightData {
    counts: i32,
    heights: Vec<i32>,
}

/// Reads in a CSV file using the include_str macro, returning a Result containing a Vector of
/// Record structs.
/// Returns Ok(Vec<ReadRecord>).
fn read_using_include_str() -> Result<Vec<ReadRecord>, Box<dyn Error>> {
    println!("Reading the file using include_str macro...");

    let mut num_obs: i32 = 0;
    let mut csv_records: Vec<ReadRecord> = Vec::new();
    // let mut csv_records: Vec<ReadRecord> = Vec::with_capacity(MAX_RECORDS);

    // let puzzle =
    //     include_str!("/home/geoffc/Computer_Stuff/Python/ChatGPT/viirs_cloud_top_height_trunc.csv")
    //         .lines();
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

        // println!("{:12.6} {:12.6} {:5}", longitude, latitude, height);
        // println!("{:?}", record);
    }

    println!("Finished looping through the lines...");
    println!("There are {:?} observations", num_obs);

    Ok(csv_records)
}

/// Reads in a CSV file, using the csv crate and manually deserializing.
/// Returns Ok(Vec<ReadRecord>).
fn read_using_csv() -> Result<Vec<ReadRecord>, Box<dyn Error>> {
    println!("Reading the file using csv crate with manual destructuring...");

    let mut num_obs: i32 = 0;
    let mut csv_records: Vec<ReadRecord> = Vec::new();
    // let mut csv_records: Vec<ReadRecord> = Vec::with_capacity(MAX_RECORDS);

    // let file_path = "/home/geoffc/Computer_Stuff/Python/ChatGPT/viirs_cloud_top_height_trunc.csv";
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

        // println!("{:12.6} {:12.6} {:5}", longitude, latitude, height);
        // println!("{:?}", record);
    }

    println!("Finished looping through the lines...");
    println!("There are {:?} observations", num_obs);

    Ok(csv_records)
}

/// Reads in a CSV file, using the csv crate and deserializing with serde crate.
/// Returns Ok(Vec<ReadRecord>).
fn read_using_csv_serde(file_path: &String) -> Result<Vec<ReadRecord>, Box<dyn Error>> {
    println!("Reading the file using csv crate with serde deserialization...");

    let mut num_obs: i32 = 0;
    let mut csv_records: Vec<ReadRecord> = Vec::new();
    // let mut csv_records: Vec<ReadRecord> = Vec::with_capacity(MAX_RECORDS);

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(file_path)?;

    for result in rdr.deserialize() {
        let record: ReadRecord = result?;

        if num_obs as usize > MAX_RECORDS {
            break;
        }
        num_obs += 1;

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

    println!("Finished looping through the lines...");
    println!("There are {:?} observations", num_obs);

    Ok(csv_records)
}

/// This function accepts as input an iterator over the lines of a string, and bins the data
/// into a 1 degree by 1 degree grid, saving the binned data in a HashMap
fn generate_histograms(
    csv_record: &Vec<ReadRecord>,
) -> Result<HashMap<String, HeightData>, Box<dyn Error>> {
    // let mut idx = 0;
    let mut grid_dict: HashMap<String, HeightData> = HashMap::new();

    let mut num_obs: i32 = 0;

    println!("Looping through the lines...");
    for (idx, record) in csv_record.iter().enumerate() {
        if idx > MAX_RECORDS {
            break;
        }
        num_obs += 1;

        let longitude = record.longitude;
        let latitude = record.latitude;
        let height = record.height;

        // Compute the grid cell coordinates for this observation
        let lon_center = GRID_SIZE * (longitude / GRID_SIZE).floor() + GRID_SIZE / 2.0;
        let lat_center = GRID_SIZE * (latitude / GRID_SIZE).floor() + GRID_SIZE / 2.0;

        // Here we are making a string key from the lat and lon center values.
        // We can also make a key from a struct containing these values, as long as they
        // derive the Eq and Hash PartialEq traits.

        let key = format!("({lon_center:6.1},{lat_center:6.1})");

        // Add this observation to the corresponding grid cell
        grid_dict
            .entry(key)
            .and_modify(|hgt| {
                hgt.counts += 1;
                hgt.heights.push(height);
            })
            .or_insert(HeightData {
                counts: 1,
                heights: vec![height],
            });
    }
    println!("Finished looping through the lines...");
    println!("There are {:?} observations", num_obs);

    Ok(grid_dict)
}

#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "PascalCase")]
struct WriteRecord {
    longitude: f64,
    latitude: f64,
    counts: i32,
    sum_heights: i32,
    sum_squared_heights: f64,
    mean_height: f64,
    stdev_height: f64,
}

/// This function reads the contents of a HashMap, computes some statistics for each key,
/// then writes the summary stats for the key (or grid cell) to a csv file.
fn calc_stats(grid_dict: &HashMap<String, HeightData>) -> Result<Vec<WriteRecord>, Box<dyn Error>> {
    println!("Calculating the stats for each grid cell...");

    let unsorted_keys: Vec<String> = grid_dict.clone().into_keys().collect();
    let mut sorted_keys = unsorted_keys.clone();
    sorted_keys.sort_unstable();

    println!(" There are {} unsorted keys...", unsorted_keys.len());
    println!(" There are {} sorted keys...", sorted_keys.len());

    // let mut csv_records: Vec<WriteRecord> = Vec::new();
    let mut csv_records: Vec<WriteRecord> = Vec::with_capacity(sorted_keys.len());

    for key in sorted_keys {

        let v: Vec<&str> = key.split(',').collect();
        let longitude = v[0].trim_matches('(').trim_matches(' ').parse::<f64>()?;
        let latitude = v[1].trim_matches(' ').trim_matches(')').parse::<f64>()?;

        // TODO: Figure out how to do this without cloning the data... need to use lifetimes
        // in the struct defn.
        let heights = &grid_dict.get(&key).unwrap().heights.clone();
        let counts = &grid_dict.get(&key).unwrap().counts.clone();

        let sum_heights: i32 = heights.iter().sum::<i32>();
        let sum_squared_heights: f64 = heights
            .iter()
            .map(|x| (*x as f64) * (*x as f64))
            .sum::<f64>();

        let mom_1: f64 = sum_heights as f64 / (*counts as f64);
        let mom_2: f64 = sum_squared_heights / (*counts as f64);
        let cum_2: f64 = mom_2 - mom_1 * mom_1;

        let mean_height: f64 = mom_1;
        let stdev_height: f64 = cum_2.sqrt();

        csv_records.push(
            WriteRecord {
                longitude ,
                latitude,
                counts: *counts,
                sum_heights,
                sum_squared_heights,
                mean_height,
                stdev_height
            }
        );
    }
    Ok(csv_records)
}

/// This function accepts as input an iterator over the lines of a string, and bins the data
/// into a 1 degree by 1 degree grid, saving the binned data in a HashMap
fn write_csv_using_serde(csv_record: &Vec<WriteRecord>) -> Result<(), Box<dyn Error>> {
    let path = "./data/viirs_stats-2_rs.csv";

    let mut wtr = csv::Writer::from_path(path)?;

    for record in csv_record {
        wtr.serialize(record)?;
    }

    wtr.flush()?;

    Ok(())
}
/// The main function
///
/// There are bunch of ways that I could put this together, but I think that the functions should
/// be..
///
/// 1. Takes a &file_path, opens and reads file using csv crate with serde support, returns a Vec<ReadRecord>
/// 2. Takes &Vec<ReadRecord>, grids the height data into, and returns, a HashMap<String, HeightData>
/// 3. Takes a &HashMap<String, HeightData> of gridded data, computes stats for each key, which are collected
///    and returned as Vec<WriteRecord>
/// 4. Takes a &Vec<WriteRecord>, opens a new file, and serializes the HashMap to the file using the csv
///    crate and serde
///
/// TODO: Better error handling
/// TODO: Sensible exit codes
/// TODO: Input argument parsing using clap
/// TODO: Use of modules
///
fn main() -> Result<(), Box<dyn Error>> {
    println!("GRID_SIZE = {GRID_SIZE}");

    // let file_path = "/home/geoffc/Computer_Stuff/Python/ChatGPT/viirs_cloud_top_height_trunc.csv".to_string();
    let file_path = "./data/viirs_cloud_top_height.csv".to_string();

    // let result = read_using_include_str()?;
    // let result = read_using_csv()?;
    let csv_records = read_using_csv_serde(&file_path)?;

    let grid_dict = generate_histograms(&csv_records)?;

    let output_records = calc_stats(&grid_dict)?;

    let _ = write_csv_using_serde(&output_records)?;

    Ok(())
}
