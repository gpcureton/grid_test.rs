use std:: error::Error;

use clap::{
    // arg,
    // command,
    // value_parser,
    ArgAction,
    Arg,
    Command, ArgMatches
};

pub fn args() -> Result<ArgMatches, Box<dyn Error>> {
    // println!("This is where we would sort out the args.");

    let cmd = Command::new("grid_test")
        .author("Geoff Cureton, geoff.cureton@ssec.wisc.edu")
        .about("Test program for gridding lon/lat data")
        // .help_template("\
        // {before-help}{name} {version}
        // {author-with-newline}{about-with-newline}
        // {usage-heading} {usage}

        // {all-args}{after-help}
        // ")
        .arg(Arg::new("in_file")
            .short('i')
            .long("input")
            .value_name("FILE [FILE1, FILE2, ...]")
            .required(true)
            .action(ArgAction::Set)
            .num_args(1..)
            .help("Input csv file(s)."))
        .arg(Arg::new("out_file")
            .short('o')
            .long("output")
            .value_name("FILE")
            .required(true)
            .action(ArgAction::Set)
            .num_args(1)
            .help("Output csv file."))
        .arg(Arg::new("grid_size")
            .short('g')
            .long("gridsize")
            .required(false)
            .num_args(1)
            .action(ArgAction::Set)
            .value_parser(clap::value_parser!(f64))
            .default_value("1.0")
            .allow_negative_numbers(false)
            .help("Longitude/Latitude grid size in degrees."))
        .get_matches();

    Ok(cmd)
}
