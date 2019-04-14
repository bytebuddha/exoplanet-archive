extern crate serde_json;
extern crate reqwest;

mod microlensing;
pub use self::microlensing::MicrolensingExoplanet;

mod confirmed;
pub use self::confirmed::ConfirmedExoplanet;

mod error;
pub use self::error::Error;

pub type Result<T> = ::std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
