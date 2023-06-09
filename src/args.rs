use std::{error::Error, path::PathBuf};

use clap::{
    // arg,
    // command,
    builder::PossibleValue,
    Arg,
    ArgAction,
    ArgMatches,
    Command,
};

// #[derive(Debug)]
// enum Algorithm {
//     Csv = "csv",
//     Serde = "serde"
// }

/// This method examines the input args from sys.args[] and determines
/// whether the expert flag is set...
pub fn is_expert(sys_args: &[String]) -> Result<bool, Box<dyn Error>> {
    let mut is_expert = false;

    let mut sys_args_list = sys_args.to_owned().split_off(1);
    sys_args_list.sort_unstable();

    sys_args_list.dedup();

    let expert_long_idx = sys_args_list.iter().position(|r| r == "--expert");

    let expert_short_idx = sys_args_list.iter().position(|r| r == "-x");

    // if (expert_long_idx != None) || (expert_short_idx != None) {
    if (expert_long_idx.is_some()) || (expert_short_idx.is_none()) {
        // log::info!("We have an expert option!");
        is_expert = true;
    }

    if is_expert {
        sys_args_list.extend(vec!["--help".to_string()]);
        sys_args_list.retain(|x| x != "-x" && x != "--expert");
    }
    sys_args_list.dedup();

    Ok(is_expert)
}

/// This function collects and handles the command line args using clap.
pub fn args(sys_args: &[String]) -> Result<ArgMatches, Box<dyn Error>> {
    // log::info!("Input arguments {sys_args:?}");

    let expert_args: bool = is_expert(sys_args)?;

    log::info!("args::args initial sys_args are: {sys_args:?}");

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
            .value_parser(clap::value_parser!(PathBuf))
            .num_args(1..)
            .help("Input csv file(s)."))
        .arg(Arg::new("out_file")
            .short('o')
            .long("output")
            .value_name("FILE")
            .required(true)
            .action(ArgAction::Set)
            .value_parser(clap::value_parser!(PathBuf))
            .num_args(1)
            .help("Output csv file."))
        .arg(Arg::new("satellite")
            .long("satellite")
            .value_name("SATELLITE")
            .required(false)
            .action(ArgAction::Set)
            .hide(expert_args)
            .num_args(1)
            .value_parser([
                PossibleValue::new("him8").help("Himawari-8"),
                PossibleValue::new("him9").help("Himawari-9")])
            .help("The satellite to run grid_test on. This is only required if automatic detection of the satellite fails."))
        .arg(Arg::new("alg")
            .long("alg")
            .value_name("ALG")
            .required(false)
            .action(ArgAction::Set)
            .num_args(1)
            .value_parser([
                PossibleValue::new("csv")
                    .help("Read input file(s) using csv crate with manual destructuring"),
                PossibleValue::new("serde")
                    .help("Read input file(s) using csv crate with serde deserialization")])
                // TODO: Determine whether we can use enums for this...
                // PossibleValue::new(Algorithm::Csv).help("Read input file(s) using csv crate with manual destructuring"),
                // PossibleValue::new(Algorithm::Serde).help("Read input file(s) using csv crate with serde deserialization")])
            .default_value("serde")
            .hide(expert_args)
            .help("Algorithm to use for reading input csv file."))
        .arg(Arg::new("grid_size")
            .short('g')
            .long("gridsize")
            .required(false)
            .num_args(1)
            .action(ArgAction::Set)
            .value_parser(clap::value_parser!(f64))
            .default_value("1.0")
            .hide(expert_args)
            .allow_negative_numbers(false)
            .help("Longitude/Latitude grid size in degrees."))
        .arg(Arg::new("verbosity")
            .short('v')
            .long("verbosity")
            .action(ArgAction::Count)
            .value_parser(clap::value_parser!(u8))
            // .default_value("2")
            .help("Each occurrence increases verbosity 1 level from ERROR: -v=WARNING, -vv=INFO, -vvv=DEBUG"))
        .arg(Arg::new("log_file")
            .short('l')
            .long("logfile")
            .value_name("FILE")
            .required(false)
            .action(ArgAction::Set)
            .value_parser(clap::value_parser!(PathBuf))
            .num_args(1)
            // .default_value("grid_test.log")
            .hide(expert_args)
            .help("Output log file."))
        .arg(Arg::new("expert")
            .short('x')
            .long("expert")
            .action(ArgAction::SetTrue)
            .value_parser(clap::value_parser!(bool))
            .help("Display all help options, including the expert ones."));

    Ok(cmd.get_matches())
}
