//! Library for parsing data from the [exoplanetAPI](https://exoplanetarchive.ipac.caltech.edu)
//!

extern crate serde_json;
extern crate reqwest;

mod microlensing;
pub use self::microlensing::MicrolensingExoplanet;

mod confirmed;
pub use self::confirmed::ConfirmedExoplanet;

mod error;
pub use self::error::Error;

pub type Result<T> = ::std::result::Result<T, Error>;

pub trait ApiEndpoint: Sized {

    const BASE_URL: &'static str = "https://exoplanetarchive.ipac.caltech.edu/cgi-bin/nstedAPI/nph-nstedAPI";

    const TABLE_NAME: &'static str;

    fn handle_data(data: &str) -> Result<Vec<Self>>;

    fn load() -> Result<Vec<Self>> {
        let url = format!("{}?table={}&format=json", Self::BASE_URL, Self::TABLE_NAME);
        let data = reqwest::get(&url)?.text()?;
        Ok(Self::handle_data(&data)?)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
