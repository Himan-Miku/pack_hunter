mod functions;
mod structs;

use clap::Parser;
use functions::{javascript, python, rust};
use structs::cli::{Cli, Commands};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Javascript(js_options) => {
            javascript::search_pack(js_options).await.unwrap();
        }
        Commands::Rust(rs_options) => {
            rust::search_pack(rs_options).await;
        }
        Commands::Python(py_options) => {
            python::search_pack(py_options).await;
        }
    }
}
