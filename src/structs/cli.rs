use crate::functions::{javascript, rust};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Search for Javascript Packages
    Javascript(javascript::JsOptions),
    /// Search for Rust Crates
    Rust(rust::RsOptions),
}
