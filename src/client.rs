use std::fs;
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

    pub fn repo(&self) -> Result<PathBuf,anyhow::Error> {

        let mut path = PathBuf::new();
        path.push( format!("{}/.broadside",dirs::home_dir().ok_or(anyhow!("cannot determine home_dir"))?.to_str().ok_or(anyhow!("cannot convert homedir to_str"))?));
        Ok(path)
    }

    pub fn account_dir(&self) -> Result<PathBuf,anyhow::Error> {
        let mut path = self.repo()?;

        path.push("accounts");

        Ok(path)
    }


    pub fn publish( &self, path: PathBuf) -> Result<(), anyhow::Error>
    {

        fs::create_dir_all(self.account_dir()?)?;


        println!( "$HOME {}", dirs::home_dir().unwrap().to_str().unwrap());

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
