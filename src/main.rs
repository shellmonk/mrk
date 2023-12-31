use clap::Parser;
use crate::cli::CliArgs;

mod cli;
mod generator;
mod config;

fn main() {
    let config = CliArgs::parse();

    // TODO: Shitty design, fix this
    generator::generate(&config.config, &config::parse_config(&config.config));
}
