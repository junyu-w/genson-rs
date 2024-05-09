use std::process;

use serde_json::Value;
use clap::Parser;
use genson_rs::get_builder;


#[derive(Parser)]
#[command(name = "Genson")]
#[command(version = "0.1")]
#[command(about = "Generate one, unified JSON Schema from one or \
more JSON objects and/or JSON Schemas. Compatible with JSON-Schema\
 Draft 4 and above.", long_about = None)]
struct Cli {
    #[arg(short, long)]
    delimiter: Option<String>,

    #[arg(short, long)]
    encoding: Option<String>,

    #[arg(short, long)]
    schema_file: Option<String>,

    json_file: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let mut builder = get_builder(Some("AUTO"));

    if let Some(json_file) = cli.json_file.as_deref() {
        let object_str = std::fs::read_to_string(json_file).unwrap();
        let object: Value = serde_json::from_str(&object_str).unwrap();

        builder.add_object(&object);
        println!("{}", builder.to_json());
        // NOTE: early exit here to avoid dropping of the `object` variable
        //  which takes about 15~35% of the total runtime (depending on the size of the object)
        process::exit(0);
    }
}
