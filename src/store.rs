use std::fs;
use std::io::Write;
use std::os::unix::prelude::OpenOptionsExt;
use std::path::PathBuf;
use acid_store::repo::key::KeyRepo;
use acid_store::repo::{OpenMode, OpenOptions};
use acid_store::store::DirectoryConfig;
use crate::model::CannonBall;

pub struct Store {
   repo: KeyRepo<String>
}

impl Store {

    fn repo_dir() -> Result<PathBuf,anyhow::Error> {
        let mut path = PathBuf::new();
        path.push( format!("{}/.broadside",dirs::home_dir().ok_or(anyhow!("cannot determine home_dir"))?.to_str().ok_or(anyhow!("cannot convert homedir to_str"))?));
        Ok(path)
    }

    fn account_dir() -> Result<PathBuf,anyhow::Error> {
        let mut path = Self::repo_dir()?;
        path.push("accounts");
        Ok(path)
    }

    pub fn new() -> Result<Self,anyhow::Error> {
        let config = DirectoryConfig {
            path: Self::account_dir()?
        };
        let mut repo = OpenOptions::new().mode(OpenMode::Create).open(&config)?;
        Ok(Store{repo})
    }


    pub fn save( & mut self, cannonball: CannonBall, data: Vec<u8>) -> Result<(),anyhow::Error> {
        let mut object = self.repo.insert(cannonball.to_string());
        object.write(data.as_slice())?;
        object.commit()?;
        Ok(())
    }



}