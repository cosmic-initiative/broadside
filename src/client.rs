use std::io::Cursor;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use crate::zip::zip_dir;

pub struct Client {

}

impl Client {

    pub fn new() -> Self {
        Client{}
    }

    pub fn publish( &self, path: PathBuf) -> Result<(), anyhow::Error>
    {

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
        Ok(())
    }
}
