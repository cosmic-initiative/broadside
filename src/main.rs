pub mod client;
pub mod zip;
pub mod parse;
pub mod model;
pub mod store;

use std::{fs, io::{Cursor, Read, Seek, Write}, io, path::Path};
use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;
use clap::{Args, Parser, Subcommand, ValueEnum};
use walkdir::{DirEntry, WalkDir};
use ::zip::write::FileOptions;
use crate::client::Client;
use crate::model::CannonBall;
use crate::parse::cannonball_complete;

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
      path: PathBuf,
      cannonball: String
  }
}




#[tokio::main]
async fn main() -> Result<(),anyhow::Error>{
    let args = Cli::parse();
    match args.command {
        Command::Publish { path, cannonball } => {
            let cannonball = CannonBall::from_str(cannonball.as_str())?;
println!("CAnnon BALL: {}", cannonball.to_string());
            let client = Client::new()?;
            client.publish(path)?;
        }
    }

    Ok(())
}
