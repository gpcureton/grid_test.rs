use std::error::Error;
use std::collections::HashMap;

pub const GRID_SIZE: f64 = 1.0;
// pub const GRID_SIZE: f64 = 0.25;
// const MAX_RECORDS: usize = 500;
pub const MAX_RECORDS: usize = 10_000_000;
// const MAX_RECORDS: usize = std::usize::MAX;

use crate::inputs::ReadRecord as ReadRecord;
use crate::outputs::WriteRecord as WriteRecord;

/// The HeightData struct holds binned observations of a gridcell
/// Also stored is the number of observations for that gridcell
#[derive(Debug, Clone, PartialEq)]
pub struct HeightData {
    counts: i64,
    heights: Vec<i64>,
}

/// This function accepts as input an iterator over the lines of a string, and bins the data
/// into a 1 degree by 1 degree grid, saving the binned data in a HashMap
pub fn generate_histograms( csv_records: &[ReadRecord], grid_size: &f64) -> Result<HashMap<String, HeightData>, Box<dyn Error>> {
    println!("Binning the csv records into a histogram...");

    // let mut idx = 0;
    let mut grid_dict: HashMap<String, HeightData> = HashMap::new();

    let mut num_records: i64 = 0;

    for (_idx, record) in csv_records.iter().enumerate() {

        num_records += 1;

        let longitude = record.longitude;
        let latitude = record.latitude;
        let height = record.height;

        // Compute the grid cell coordinates for this observation
        // let lon_center = GRID_SIZE * (longitude / GRID_SIZE).floor() + GRID_SIZE / 2.0;
        // let lat_center = GRID_SIZE * (latitude / GRID_SIZE).floor() + GRID_SIZE / 2.0;
        let lon_center = grid_size * (longitude / grid_size).floor() + grid_size / 2.0;
        let lat_center = grid_size * (latitude / grid_size).floor() + grid_size / 2.0;

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
    println!("Finished binning the csv records.");
    println!("There are {:?} csv records\n", num_records);

    Ok(grid_dict)
}

/// This function reads the contents of a HashMap, computes some statistics for each key,
/// then writes the summary stats for the key (or grid cell) to a csv file.
pub fn calc_stats(grid_dict: &HashMap<String, HeightData>) -> Result<Vec<WriteRecord>, Box<dyn Error>> {
    println!("Calculating the stats for each grid cell...");

    let unsorted_keys: Vec<String> = grid_dict.clone().into_keys().collect();
    let mut sorted_keys = unsorted_keys.clone();
    sorted_keys.sort_unstable();

    println!("There are {} unsorted keys...", unsorted_keys.len());
    println!("There are {} sorted keys...", sorted_keys.len());

    let num_keys = sorted_keys.len();

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

        // let sum_heights: i64 = heights.iter().sum::<i64>();
        let sum_heights: i64 = heights.iter().sum();
        let sum_squared_heights: i64 = heights
            .iter()
            .map(|x| (*x) * (*x))
            .sum();

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

    println!("Finished calculating the stats for each grid cell.");
    println!("There are {:?} grid cells\n", num_keys);

    Ok(csv_records)
}

#[cfg(test)]
mod tests {
    use crate::HeightData;

    #[test]
    /// This test checks that the struct attributes are the values
    /// they were defined as.
    fn heights_struct_test() {
        let cell_1 = HeightData {
            counts: 25,
            heights: vec![1266, 12656, 5256, 735],
        };
        let cell_2 = cell_1.clone();
        assert_eq!(cell_1, cell_2);
    }
}
