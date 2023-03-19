use crate::dirs::cache_dir;
use crate::model::CannonFile;
use crate::store::Store;

pub struct Cache {
  store: Store
}

impl Cache {

    pub fn new() -> Result<Self,anyhow::Error> {
        let store = Store::new()?;
        Ok(Cache{store})
    }

    pub fn get(&self, file: &CannonFile ) -> Result<Vec<u8>,anyhow::Error> {
        let mut cache_dir= cache_dir()?;
        cache_dir.push(file.to_string());
        println!("cache_dir: {}", cache_dir.to_str().unwrap());
        unimplemented!();
    }

}