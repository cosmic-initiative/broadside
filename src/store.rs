use std::fs;
use std::path::PathBuf;

pub struct Store {

}

impl Store {

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

    pub fn new() -> Self {
        Store{}
    }

    pub fn init(&self) -> Result<(),anyhow::Error> {
        fs::create_dir_all(self.account_dir()?)?;
        Ok(())
    }



}