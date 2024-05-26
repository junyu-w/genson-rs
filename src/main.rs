use std::process;

use clap::{ArgAction, Parser};
use genson_rs::*;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::mem;

#[derive(Parser)]
#[command(name = "genson-rs")]
#[command(version = "0.2")]
#[command(about = "Generate one, unified JSON Schema from JSON objects. Compatible with JSON Schema Draft-4 and above.", long_about = None)]
#[command(author = "Junyu Wang <wjyu95@gmail.com>")]
#[command(arg_required_else_help = true)]
struct Cli {
    #[arg(short, long)]
    /// Must be one of "newline", "tab", "space". 
    /// Specifying a delimiter is optional, but will improve the performance 
    /// if your input is multiple JSON objects concatenated together (e.g. each object on a newline)
    delimiter: Option<String>,

    #[arg(short, long, action=ArgAction::SetTrue, default_value="false")]
    /// Only applicable if you JSON file is one JSON arrary, and  
    /// you only care about the schema of the JSON objects inside of it
    ignore_outer_array: bool,

    /// Path to the JSON file(s) to generate the schema from. The generated schema will 
    /// accomodate all the JSON objects in the file(s).
    json_files: Option<Vec<String>>,
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

/// Generate a JSON Schema from a JSON file
fn build_schema(
    builder: &mut SchemaBuilder, file_path: &str, config: BuildConfig
) {
    let mut object_slice = std::fs::read(file_path).unwrap();
    build_json_schema(builder, &mut object_slice, &config);
    // NOTE: avoid dropping the object_slice to improve performance
    // the effect is more siginificant for larger JSON files
    mem::forget(object_slice);
}

fn main() {
    let cli = Cli::parse();
    let delimiter = get_delimiter(&cli);

    if let Some(json_files) = cli.json_files.as_deref() {
        // parallelize the schema building process for multiple JSON files
        let aggregated_builder = json_files.par_iter()
            .fold(
            || get_builder(Some("AUTO")), 
            |mut builder, file_path| {
                build_schema(&mut builder, file_path, BuildConfig {
                    delimiter,
                    ignore_outer_array: cli.ignore_outer_array,
                });
                return builder;
            }).reduce(
            || get_builder(Some("AUTO")),
            |mut builder, other_builder| {
                builder.add_schema(other_builder.to_schema());
                return builder;
            });

        let schema = aggregated_builder.to_schema();
        println!("{}", schema.to_string());

        // NOTE: early exit here to avoid dropping of the `object` variable
        //  which takes about 15~35% of the total runtime (depending on the size of the object)
        process::exit(0);
    } else {
        println!("No JSON files provided. Use --help for more information.");
        process::exit(1);
    }
}
