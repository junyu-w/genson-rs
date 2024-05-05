use clap::Parser;
use genson_rs::{add_object_file, get_builder};

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
    let mut builder = get_builder(None);

    if let Some(json_file) = cli.json_file.as_deref() {
        println!("JSON file: {}", json_file);
        add_object_file(&mut builder, json_file)
    }

    println!("{}", builder.to_json());
}
