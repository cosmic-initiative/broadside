use std::path::PathBuf;

pub fn broadside_store() -> Result<PathBuf,anyhow::Error> {
    let mut path = PathBuf::new();
    path.push( format!("{}/.broadside",dirs::home_dir().ok_or(anyhow!("cannot determine home_dir"))?.to_str().ok_or(anyhow!("cannot convert homedir to_str"))?));
    Ok(path)
}

pub fn accounts_dir() -> Result<PathBuf,anyhow::Error> {
    let mut path = broadside_store()?;
    path.push("accounts");
    Ok(path)
}

pub fn cache_dir() -> Result<PathBuf,anyhow::Error> {
    let mut path = broadside_store()?;
    path.push("cache");
    Ok(path)
}
