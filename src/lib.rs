use clap::{Parser, Subcommand};

pub mod javascript;
pub mod python;
pub mod rust;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Search for Python Packages
    Python(python::PyOptions),
    /// Search for Javascript Packages
    Javascript(javascript::JsOptions),
    /// Search for Rust Crates
    Rust(rust::RsOptions),
}