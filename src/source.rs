use crate::model::CannonBall;

pub trait Source {
    fn fetch(&self, cannonball: &CannonBall ) -> Result<Vec<u8>,anyhow::Error>;
}