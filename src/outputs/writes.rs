use std::error::Error;
use serde::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct WriteRecord {
    pub longitude: f64,
    pub latitude: f64,
    pub counts: i32,
    pub sum_heights: i32,
    pub sum_squared_heights: i64,
    pub mean_height: f64,
    pub stdev_height: f64,
}

/// This function accepts as input an iterator over the lines of a string, and bins the data
/// into a 1 degree by 1 degree grid, saving the binned data in a HashMap
pub fn write_csv_using_serde(csv_records: &Vec<WriteRecord>) -> Result<(), Box<dyn Error>> {
    println!("Serializing the histogram data to file...");

    let path = "./data/viirs_cloud_top_height_stats.csv";

    let mut wtr = csv::Writer::from_path(path)?;

    let mut num_grids_cells: i32 = 0;

    for record in csv_records {
        wtr.serialize(record)?;
        num_grids_cells += 1;
    }

    wtr.flush()?;

    println!("Finished serializing the histogram data to a csv file...");
    println!("There are {:?} entries in the csv file.\n", num_grids_cells);

    // Err("There was an error writing to the CSV file")?
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::WriteRecord;
    #[test]
    /// This test checks that the struct attributes are the values
    /// they were defined as.
    fn writer_struct_test() {
        let record = WriteRecord{
            longitude: 100.0,
            latitude: 35.2,
            counts: 25,
            sum_heights: 123456,
            sum_squared_heights: 23456789123,
            mean_height: 23456.0,
            stdev_height: 123.5,
        };
        assert_eq!(record.longitude, 100.0);
        assert_eq!(record.latitude, 35.2);
        assert_eq!(record.counts, 25);
        assert_eq!(record.sum_heights, 123456);
        assert_eq!(record.sum_squared_heights, 23456789123);
        assert_eq!(record.mean_height, 23456.0);
        assert_eq!(record.stdev_height, 123.5);
    }

    #[test]
    /// This test compares a struct with a clone of itself, which is possible
    /// as the struct derives the PartialEq trait
    fn write_struct_equality(){
        let record_1 = WriteRecord{
            longitude: 100.0,
            latitude: 35.2,
            counts: 25,
            sum_heights: 123456,
            sum_squared_heights: 23456789123,
            mean_height: 23456.0,
            stdev_height: 123.5,
        };
        let record_2 = record_1.clone();
        assert_eq!(record_1, record_2);
    }
}
