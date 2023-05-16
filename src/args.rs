use std:: error::Error;

// use clap::{Arg, Parser, ArgAction};
// use clap::{Arg, Command, ArgAction};

// #[derive(Parser, Debug)]
// #[command(author, version, about, long_about = None)]
// struct Args {
//     /// Input csv file with (lon, lat, height) columns
//     #[arg(short, long)]
//     name: String,

//     /// Number of records to read in
//     #[arg(short, long, default_value_t = 1)]
//     num_records: u32,

// }

pub fn args() -> Result<(), Box<dyn Error>> {
    println!("This is where we would sort out the args.");

    // let matches = Command::new("grid_test")
    //     .author("Geoff Cureton, geoff.cureton@ssec.wisc.edu")
    //     .version("0.1.0")
    //     .about("Test program for gridding lon/lat data")
    //     .arg(
    //         Arg::new("in_file")
    //         .short('i')
    //         .long("input")
    //         .required(true)
    //         .action(ArgAction::Set)
    //         .default_value("-")
    //         .help("Input csv file of longitudes, latitudes and heights.")
    //     )
    //     .arg(
    //         Arg::new("num_records")
    //         .short('n')
    //         .long("num-records")
    //         .required(false)
    //         .action(ArgAction::Set)
    //         // .default_value()
    //         .help("Number of records to read in.")
    //     )
    //     .after_help("This is some test to appear after the options.")
    //     .get_matches();

// }
    // to get information about the "cfg" argument we created, such as the value supplied we use
    // various ArgMatches methods, such as [ArgMatches::get_one]
    let _num_records = grid_test::MAX_RECORDS;

    // if let Some(n) = matches.get_one::<String>("num_records") {
    //     println!("Value for -n: {}", n);
    //     // num_records = num_records.parse::<i32>().unwrap();
    // }
    // println!("Input num_records: {}", num_records);

    // The ArgMatches::get_one method returns an Option because the user may not have supplied
    // that argument at runtime. But if we specified that the argument was "required" as we did
    // with the "out" argument, we can safely unwrap because `clap` verifies that was actually
    // used at runtime.
    // let file_path = matches.get_one::<String>("in_file").unwrap();
    // println!("Value for --input: {}", file_path);

    Ok(())
}
