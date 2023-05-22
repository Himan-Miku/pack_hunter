mod functions;
mod structs;

use clap::Parser;
use structs::cli::{Cli, Commands};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Javascript(js_options) => {
            javascript::search_pack(js_options).await.unwrap();
        }
        Commands::Rust(_rs_options) => {
            println!("Rust")
        }
        Commands::Python(_py_options) => {
            println!("Python")
        }
    }
}
