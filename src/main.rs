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
    verbose: Option<bool>,

    json_file: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let mut builder = get_builder(Some("AUTO"));

    if let Some(json_file) = cli.json_file.as_deref() {
        let now = Instant::now();

        // let object = parse_json_object(json_file);
        let mut object_slice = std::fs::read(json_file).unwrap();

        if cli.verbose.unwrap_or(false) {
            eprintln!("File reading duration: {}ms", now.elapsed().as_millis());
        }
        parse_json_schema(&mut builder, &mut object_slice, cli.verbose.unwrap_or(false));
        
        println!("{}", builder.to_json());
        // NOTE: early exit here to avoid dropping of the `object` variable
        //  which takes about 15~35% of the total runtime (depending on the size of the object)
        process::exit(0);
    } else {
        <Cli as CommandFactory>::command().print_help().unwrap();
    }
}
