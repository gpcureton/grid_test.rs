use std::error::Error;
// use csv::DeserializeError;
use serde::Deserialize;

// use crate::{MAX_RECORDS, GRID_SIZE};
use crate::MAX_RECORDS;

/// The Record struct holds a single line of data read from a csv file
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ReadRecord {
    pub longitude: f64,
    pub latitude: f64,
    pub height: i64,
}

/// Reads in a CSV file, using the csv crate and deserializing with serde crate.
/// Returns Ok(Vec<ReadRecord>).
pub fn read_using_csv_serde(
    files: &Vec<&String>,
    max_records: &usize,
) -> Result<Vec<ReadRecord>, Box<dyn Error>> {
    // let mut csv_records: Vec<ReadRecord> = Vec::new();
    let mut csv_records: Vec<ReadRecord> = Vec::with_capacity(MAX_RECORDS);

    for file_path in files {
        println!("Reading the file '{file_path}' using csv crate with serde deserialization...");

        let mut num_records: i64 = 0;

        // Dereference
        let file_path = *file_path;

        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(file_path)?;

        for result in rdr.deserialize() {
            let record: ReadRecord = result?;

            if num_records as usize > *max_records {
                println!("Breaking at {}", num_records);
                break;
            }
            num_records += 1;

            csv_records.push(record);
        }

        println!("\tFinished deserializing the csv file...");
        println!("\tThere are {:?} entries in the csv file.\n", num_records);
    }

    println!("Finished reading the csv files...");
    println!("There are {:?} total entries read.\n", csv_records.len());

    // Err("This is an error")?
    Ok(csv_records)
}

/// Reads in a CSV file, using the csv crate and manually deserializing.
/// Returns Ok(Vec<ReadRecord>).
pub fn read_using_csv(
    files: &Vec<&String>,
    max_records: &usize,
) -> Result<Vec<ReadRecord>, Box<dyn Error>> {
    // let mut csv_records: Vec<ReadRecord> = Vec::new();
    let mut csv_records: Vec<ReadRecord> = Vec::with_capacity(MAX_RECORDS);

    for file_path in files {
        println!("Reading the file '{file_path}' using csv crate with manual destructuring...");

        let mut num_records: i64 = 0;

        // Dereference
        let file_path = *file_path;

        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(file_path)?;

        for result in rdr.records() {
            let record = result?;

            if num_records as usize > *max_records {
                println!("Breaking at {}", num_records);
                break;
            }
            num_records += 1;

            let longitude: f64 = record[0].parse()?;
            let latitude: f64 = record[1].parse()?;
            let height: i64 = record[2].parse()?;

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
        println!("There are {:?} observations", num_records);
    }

    println!("Finished reading the csv files...");
    println!("There are {:?} total entries read.\n", csv_records.len());

    Ok(csv_records)
}

#[cfg(test)]
mod tests {
    use crate::ReadRecord;

    #[test]
    /// This test checks that the struct attributes are the values
    /// they were defined as.
    fn reader_struct_test() {
        let record = ReadRecord {
            longitude: 100.0,
            latitude: 35.2,
            height: 12345,
        };
        assert_eq!(record.longitude, 100.0);
        assert_eq!(record.latitude, 35.2);
        assert_eq!(record.height, 12345);
    }

    #[test]
    /// This test compares a struct with a clone of itself, which is possible
    /// as the struct derives the PartialEq trait
    fn read_struct_equality() {
        let record_1 = ReadRecord {
            longitude: 100.0,
            latitude: 35.2,
            height: 12345,
        };
        let record_2 = record_1.clone();
        assert_eq!(record_1, record_2);
    }
}
