pub mod err;

use std::{fs, io::{Cursor, Read, Seek, Write}, io, path::Path};
use std::fs::File;
use std::path::PathBuf;
use clap::{Args, Parser, Subcommand, ValueEnum};
use walkdir::{DirEntry, WalkDir};
use zip::write::FileOptions;

#[macro_use]
extern crate tokio;

#[macro_use]
extern crate anyhow;

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "packx")]
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
            println!("Publishing Dir: {}", path.to_str().unwrap());

            let metadata = std::fs::metadata(&path)?;

            let content = if metadata.is_dir() {
                let file = Cursor::new(Vec::new());

                let walkdir = WalkDir::new(&path);
                let it = walkdir.into_iter();

                let data = match zip_dir(
                    &mut it.filter_map(|e| e.ok()),
                    path.to_str().unwrap(),
                    file,
                    zip::CompressionMethod::Deflated,
                ) {
                    Ok(data) => data,
                    Err(e) => return Err(anyhow!(format!("{}",e.to_string())))
                };

                // return the inner buffer from the cursor
                let data = data.into_inner();
                data
            } else {
                std::fs::read(&path)?
            };
println!("content: {}", content.len());
        }
    }

    Ok(())
}

fn zip_dir<T>(
    it: impl Iterator<Item = DirEntry>,
    prefix: &str,
    writer: T,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<T>
    where
        T: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            zip.start_file(name.to_str().unwrap(), options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            zip.add_directory(name.to_str().unwrap(), options)?;
        }
    }
    let result = zip.finish()?;
    Result::Ok(result)
}
