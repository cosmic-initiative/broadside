use semver::Version;

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