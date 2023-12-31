use clap::Parser;
use crate::cli::CliArgs;

mod cli;
mod generator;
mod config;

fn main() {
    let config = CliArgs::parse();
    generator::generate(&config.config, &config::parse_config(&config.config));
}
