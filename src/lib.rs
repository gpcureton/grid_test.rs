/*
use std::error::Error;
use std::collections::HashMap;

pub const GRID_SIZE: f64 = 1.0;
// const MAX_RECORDS: usize = 500;
pub const MAX_RECORDS: usize = 10_000_000;
// const MAX_RECORDS: usize = std::usize::MAX;

use serde::Serialize;

pub mod reads;

use reads::reads::ReadRecord as ReadRecord;
// pub use reads::ReadRecord as ReadRecord;

/// The HeightData struct holds binned observations of a gridcell
/// Also stored is the number of observations for that gridcell
#[derive(Clone, Debug)]
pub struct HeightData {
    counts: i32,
    heights: Vec<i32>,
}

/// This function accepts as input an iterator over the lines of a string, and bins the data
/// into a 1 degree by 1 degree grid, saving the binned data in a HashMap
// pub fn generate_histograms( csv_records: &[ReadRecord],) -> Result<HashMap<String, HeightData>, Box<dyn Error>> {
pub fn generate_histograms( csv_records: &Vec<ReadRecord>,) -> Result<HashMap<String, HeightData>, Box<dyn Error>> {
    // let mut idx = 0;
    let mut grid_dict: HashMap<String, HeightData> = HashMap::new();

    let mut num_obs: i32 = 0;

    println!("Looping through the lines...");
    for (_idx, record) in csv_records.iter().enumerate() {

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
pub struct WriteRecord {
    longitude: f64,
    latitude: f64,
    counts: i32,
    sum_heights: i32,
    sum_squared_heights: i64,
    mean_height: f64,
    stdev_height: f64,
}

/// This function reads the contents of a HashMap, computes some statistics for each key,
/// then writes the summary stats for the key (or grid cell) to a csv file.
pub fn calc_stats(grid_dict: &HashMap<String, HeightData>) -> Result<Vec<WriteRecord>, Box<dyn Error>> {
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
        let sum_squared_heights: i64 = heights
            .iter()
            .map(|x| (*x as i64) * (*x as i64))
            .sum::<i64>();

        let mom_1: f64 = sum_heights as f64 / (*counts as f64);
        let mom_2: f64 = sum_squared_heights as f64 / (*counts as f64);
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
pub fn write_csv_using_serde(csv_record: &Vec<WriteRecord>) -> Result<(), Box<dyn Error>> {
    let path = "./data/viirs_cloud_top_height_stats.csv";

    let mut wtr = csv::Writer::from_path(path)?;

    for record in csv_record {
        wtr.serialize(record)?;
    }

    wtr.flush()?;

    Ok(())
}
*/
