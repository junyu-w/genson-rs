use std::process;

use clap::{ArgAction, Parser};
use genson_rs::*;

#[derive(Parser)]
#[command(name = "genson-rs")]
#[command(version = "0.1")]
#[command(about = "Generate one, unified JSON Schema from JSON objects. Compatible with JSON-Schema\
 Draft 4 and above.", long_about = None)]
#[command(author = "Junyu Wang <wjyu95@gmail.com>")]
#[command(arg_required_else_help = true)]
struct Cli {
    #[arg(short, long)]
    /// Must be one of "newline", "tab", "space". 
    /// Specifying a delimiter is optional, but will improve the performance 
    /// if your input is multiple JSON objects concatenated together (e.g. each object on a newline)
    delimiter: Option<String>,

    /// The JSON file path to read the JSON objects from
    json_file: Option<String>,

    #[arg(short, long, action=ArgAction::SetTrue, default_value="false")]
    /// Only applicable if you JSON file is one JSON arrary, and  
    /// you only care about the schema of the JSON objects inside of it
    ignore_outer_array: bool,
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
        let mut object_slice = std::fs::read(json_file).unwrap();
        let config = BuildConfig {
            delimiter,
            ignore_outer_array: cli.ignore_outer_array,
        };
        let schema = build_json_schema(&mut builder, &mut object_slice, &config);
        println!("{}", schema.to_string());

        // NOTE: early exit here to avoid dropping of the `object` variable
        //  which takes about 15~35% of the total runtime (depending on the size of the object)
        process::exit(0);
    }
}
