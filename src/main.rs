pub mod store;
pub mod client;
pub mod zip;
pub mod parse;
mod model;

use std::{fs, io::{Cursor, Read, Seek, Write}, io, path::Path};
use std::fs::File;
use std::path::PathBuf;
use clap::{Args, Parser, Subcommand, ValueEnum};
use walkdir::{DirEntry, WalkDir};
use ::zip::write::FileOptions;
use crate::client::Client;

#[macro_use]
extern crate anyhow;

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "broadside")]
#[command(about = "CLI for packing and publishing", long_about = None)]
struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug,Subcommand)]
pub enum Command {
  #[command(arg_required_else_help = true)]
  Publish{
      path: PathBuf
  }
}




#[tokio::main]
async fn main() -> Result<(),anyhow::Error>{
    let args = Cli::parse();
    match args.command {
        Command::Publish { path } => {
            let client = Client::new();
            client.publish(path)?;
        }
    }

    Ok(())
}
