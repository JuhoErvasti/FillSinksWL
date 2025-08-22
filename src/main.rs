use std::path::Path;

use clap::{CommandFactory, Parser};

#[derive(Parser)]
#[command(arg_required_else_help = true)]
#[command(version, about, long_about = None)]
struct Cli {
    input: String,
    output: String,

    // FIXME: should be > 0
    #[arg(long = "minimum-slope", value_name = "MINIMUM_SLOPE", help = "Minimum Slope in degrees", long_help = "FIXME:")]
    minslope: Option<f64>,

    #[arg(long = "overwrite", value_name = "OVERWRITE", help = "Allow overwriting output file", long_help = "FIXME:")]
    overwrite: bool,
}

fn main() {
    let cli = Cli::parse();
    let input = Path::new(cli.input.as_str());
    let output = Path::new(cli.output.as_str());
    let minslope = cli.minslope.unwrap_or(0.1);

    if !cli.overwrite && output.exists() {
        println!("ERROR: output already exists. You can choose to overwrite with the --overwrite option.\n");
        Cli::command().print_help().unwrap();
        return ();
    }

    if !input.exists() {
        println!("ERROR: input file does not exist.\n");
        Cli::command().print_help().unwrap();
        return ();
    }

    println!(
        "input: {}, output: {}, minslope: {}",
        input.to_str().unwrap_or("SOMETHING WENT VERY WRONG"),
        output.to_str().unwrap_or("SOMETHING WENT VERY WRONG"),
        minslope,
    );

    println!("{}", cli.overwrite);
}
