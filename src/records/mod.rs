mod confirmed;
pub use self::confirmed::ConfirmedExoplanetRecord;

mod microlensing;
pub use self::microlensing::MicrolensingExoplanetRecord;

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