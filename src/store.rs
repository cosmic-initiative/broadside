use std::fs;
use std::io::{Read, Write};
use std::os::unix::prelude::OpenOptionsExt;
use std::path::PathBuf;
use acid_store::repo::key::KeyRepo;
use acid_store::repo::{Commit, OpenMode, OpenOptions};
use acid_store::store::DirectoryConfig;
use crate::dirs::accounts_dir;
use crate::model::CannonBall;
use crate::source::Source;

pub struct Store {
   repo: KeyRepo<String>
}

impl Store {



    pub fn new() -> Result<Self,anyhow::Error> {
        let config = DirectoryConfig {
            path: accounts_dir()?
        };
        let mut repo = OpenOptions::new().mode(OpenMode::Create).open(&config)?;
        Ok(Store{repo})
    }


    pub fn save( & mut self, cannonball: &CannonBall, data: Vec<u8>) -> Result<(),anyhow::Error> {
        let mut object = self.repo.insert(cannonball.to_string());
        object.write(data.as_slice())?;
        object.commit()?;


        self.repo.commit()?;

        Ok(())
    }



}

impl Source for Store {
    fn fetch( &self, cannonball: &CannonBall ) -> Result<Vec<u8>,anyhow::Error> {
        let mut object = self.repo.object( &cannonball.to_string()).ok_or(anyhow!("not found in remote store: {}", cannonball.to_string()))?;
        let mut buf = vec![];
        object.read_to_end(& mut buf)?;
        Ok(buf)
    }
}