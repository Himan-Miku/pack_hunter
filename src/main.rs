use clap::Parser;
use pack_hunter::{javascript, Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Javascript(js_options) => {
            javascript::search_pack(js_options);
        }
        Commands::Rust(rs_options) => {
            println!("Rust")
        }
        Commands::Python(py_options) => {
            println!("Python")
        }
    }
}
