// use std:: error::Error;
use std::error::Error;

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
//     IncludeStr,
//     Csv,
//     Serde
// }

pub fn args(sys_args: &[String]) -> Result<ArgMatches, Box<dyn Error>> {
    // println!("This is where we would sort out the args.");

    // println!("Input arguments {sys_args:?}");

    // TODO: Find any instances of -x or --expert, set is_expert to true, pop out the "expert" strings, them out replace them with a single instance of
    //       --help, and pass the resulting vector to the args module.

    let mut is_expert = false;

    let mut sys_args_list = sys_args.to_owned().split_off(1);
    sys_args_list.sort_unstable();

    sys_args_list.dedup();

    let expert_long_idx = sys_args_list.iter().position(|r| r == "--expert");

    let expert_short_idx = sys_args_list.iter().position(|r| r == "-x");

    // if (expert_long_idx != None) || (expert_short_idx != None) {
    if (expert_long_idx.is_some()) || (expert_short_idx.is_none()) {
        // println!("We have an expert option!");
        is_expert = true;
    }

    if is_expert {
        sys_args_list.extend(vec!["--help".to_string()]);
        sys_args_list.retain(|x| x != "-x" && x != "--expert");
    }
    sys_args_list.dedup();
    // println!("Input arguments sans --expert and -x : {sys_args_list:?}");

    // for (idx, argument) in sys_args_list.iter().enumerate() {
    //     println!("Input argument {idx}: {argument}");
    // }

    // println!("is_expert = {is_expert}");

    // let sys_args = sys_args_list;

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
        .arg(Arg::new("satellite")
            .long("satellite")
            .value_name("SATELLITE")
            .required(false)
            .action(ArgAction::Set)
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
                PossibleValue::new("include_str")
                    .help("Read input file(s) using the include_str macro"),
                PossibleValue::new("csv")
                    .help("Read input file(s) using csv crate with manual destructuring"),
                PossibleValue::new("serde")
                    .help("Read input file(s) using csv crate with serde deserialization")])
                // PossibleValue::new(Algorithm::IncludeStr).help("Read input file(s) using the include_str macro"),
                // PossibleValue::new(Algorithm::Csv).help("Read input file(s) using csv crate with manual destructuring"),
                // PossibleValue::new(Algorithm::Serde).help("Read input file(s) using csv crate with serde deserialization")])
            .default_value("serde")
            .hide(!is_expert)
            .help("Algorithm to use for reading input csv file."))
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
        .arg(Arg::new("expert")
            .short('x')
            .long("expert")
            .action(ArgAction::SetTrue)
            .value_parser(clap::value_parser!(bool))
            .help("Display all help options, including the expert ones."));
    // .get_matches();

    // let value_parser = cmd.get_arguments()
    //     .find(|a| a.get_id() == "grid_size").unwrap();
    // .get_value_parser();
    // println!("{value_parser:?}");

    Ok(cmd.get_matches())
}
