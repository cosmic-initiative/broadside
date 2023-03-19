pub mod client;
pub mod dirs;
pub mod model;
pub mod parse;
pub mod source;
pub mod store;
pub mod zip;

use crate::client::Client;
use crate::model::CannonBall;
use crate::parse::cannonball_complete;
use ::zip::write::FileOptions;
use clap::{Args, Parser, Subcommand, ValueEnum};
use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;
use std::{
    fs, io,
    io::{Cursor, Read, Seek, Write},
    path::Path,
};
use walkdir::{DirEntry, WalkDir};

#[macro_use]
extern crate anyhow;

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "broadside")]
#[command(about = "CLI for packing and publishing", long_about = None)]
struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(arg_required_else_help = true)]
    Publish { path: PathBuf, cannonball: String },
    #[command(arg_required_else_help = true)]
    Get { path: PathBuf },
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Cli::parse();
    match args.command {
        Command::Publish { path, cannonball } => {
            let cannonball = CannonBall::from_str(cannonball.as_str())?;
            let mut client = Client::new()?;
            client.publish(path, cannonball)?;
        }
        Command::Get { path } => {
            
        }
    }

    Ok(())
}
