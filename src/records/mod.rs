//! Records represent raw data recieved from the exoplanet archive API.

use crate::Result;

mod confirmed;
pub use self::confirmed::ConfirmedExoplanetRecord;

mod microlensing;
pub use self::microlensing::MicrolensingExoplanetRecord;

/// An Api Endpoint is a Requestable structure from the exoplanet archive api.
pub trait ApiEndpoint: Sized {

    /// Base url of the Exoplanet API. This should never need to be supplied manually.
    const BASE_URL: &'static str = "https://exoplanetarchive.ipac.caltech.edu/cgi-bin/nstedAPI/nph-nstedAPI";

    /// The table name this api endpoint represents.
    const TABLE_NAME: &'static str;

    /// This function is responsible for processing the fetched
    /// data
    fn handle_data(data: &str) -> Result<Vec<Self>>;

    /// Load all exoplanet data for this api endpoint.
    fn load() -> Result<Vec<Self>> {
        let url = format!("{}?table={}&format=json", Self::BASE_URL, Self::TABLE_NAME);
        let data = reqwest::get(&url)?.text()?;
        Ok(Self::handle_data(&data)?)
    }

    /// `fields` is a comma seperated list of the fields you would like to return.
    ///
    /// example: "pl_name,pl_discmethod,st_mass"
    fn select(fields: &str) -> Result<serde_json::Value> {
        let url = format!("{}?table={}&select={}&format=json", Self::BASE_URL, Self::TABLE_NAME, fields);
        let data = reqwest::get(&url)?.text()?;
        println!("{:?}", &data);
        Ok(serde_json::from_str(&data)?)
    }
}