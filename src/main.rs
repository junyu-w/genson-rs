use std::process;
use std::time::Instant;

use clap::{ArgAction, CommandFactory, Parser};
use genson_rs::*;

#[derive(Parser)]
#[command(name = "genson-rs")]
#[command(version = "0.1")]
#[command(about = "Generate one, unified JSON Schema from JSON objects. Compatible with JSON-Schema\
 Draft 4 and above.", long_about = None)]
struct Cli {
    #[arg(short, long, action=ArgAction::SetTrue)]
    /// verbose mode to print additional information
    verbose: Option<bool>,

    #[arg(short, long)]
    /// Must be one of "newline", "tab", "space". The delimiter to split the JSON objects, 
    /// specify a delimiter if your input is multiple JSON objects concatenated together 
    /// (e.g. each object on a newline).
    /// NOTE: If not specified, the input is assumed to be a single JSON object, and the parsing
    /// process will be slower if the input is a large JSON array.
    delimiter: Option<String>,

    /// The JSON file path to read the JSON objects from
    json_file: Option<String>,
}

/// Get the delimiter byte from the CLI arguments
fn get_delimiter(cli: &Cli) -> Option<u8> {
    let d = cli.delimiter.as_deref();
    if let Some(delimiter) = d {
        let delimiter = match delimiter {
            "newline" => Some(b'\n'),
            "tab" => Some(b'\t'),
            "space" => Some(b' '),
            _ => {
                panic!("Invalid delimiter: {}, must be one of \"newline\", \"tab\", \"space\"", delimiter);
            },
        };
        delimiter
    } else {
        None
    }
}

fn main() {
    let cli = Cli::parse();
    let mut builder = get_builder(Some("AUTO"));
    let delimiter = get_delimiter(&cli);

    if let Some(json_file) = cli.json_file.as_deref() {

        let now = Instant::now();
        let mut object_slice = std::fs::read(json_file).unwrap();
        if cli.verbose.unwrap_or(false) {
            eprintln!("File reading duration: {}ms", now.elapsed().as_millis());
        }
        build_json_schema(&mut builder, &mut object_slice, delimiter);

        println!("{}", builder.to_json());
        // NOTE: early exit here to avoid dropping of the `object` variable
        //  which takes about 15~35% of the total runtime (depending on the size of the object)
        process::exit(0);
    } else {
        <Cli as CommandFactory>::command().print_help().unwrap();
    }
}
