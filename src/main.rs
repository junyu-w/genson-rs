use std::process;
use std::time::Instant;

use clap::{ArgAction, CommandFactory, Parser};
use simd_json;
use genson_rs::*;
use mimalloc::MiMalloc;

// Setting the global allocator to mimalloc for more efficient memory allocation
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;


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
        let file_reading_duration_ms = now.elapsed().as_millis();

        let object = simd_json::to_borrowed_value(&mut object_slice).unwrap();
        let json_parsing_duration_ms = now.elapsed().as_millis() - file_reading_duration_ms;

        builder.add_object(&object);
        let total_duration_ms = now.elapsed().as_millis();
        let schema_building_duration_ms = total_duration_ms - json_parsing_duration_ms;

        if cli.verbose.unwrap_or(false) {
            dbg!(file_reading_duration_ms);
            dbg!(json_parsing_duration_ms);
            dbg!(schema_building_duration_ms);
            dbg!(total_duration_ms);
        }
        print!("{}", builder.to_json());
        // NOTE: early exit here to avoid dropping of the `object` variable
        //  which takes about 15~35% of the total runtime (depending on the size of the object)
        process::exit(0);
    } else {
        <Cli as CommandFactory>::command().print_help().unwrap();
    }
}
