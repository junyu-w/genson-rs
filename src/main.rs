use clap::Parser;

mod node;
mod strategy;

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

    if let Some(json_file) = cli.json_file.as_deref() {
        println!("JSON file: {}", json_file);
    }

    let node = node::SchemaNode::new();
}
