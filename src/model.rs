use std::str::FromStr;
use semver::Version;
use crate::parse::cannonball_complete;
use chumsky::Parser;
pub struct CannonBall {
    pub account: String,
    pub series: String,
    pub version: Version
}

impl CannonBall {
    pub fn new( account:String, series: String, version: Version ) -> Self {
        CannonBall {
            account,
            series,
            version
        }
    }
}

impl FromStr for CannonBall {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        cannonball_complete().parse(s).map_err(|e| anyhow!("invalid cannonball address"))
    }
}

impl ToString for CannonBall {
    fn to_string(&self) -> String {
        let mut rtn = String::new();
        rtn.push_str(self.account.as_str());
        rtn.push('/');
        rtn.push_str(self.series.as_str());
        rtn.push('/');
        rtn.push_str(self.version.to_string().as_str());
        rtn
    }
}