//! Library for parsing data from the [exoplanetAPI](https://exoplanetarchive.ipac.caltech.edu)
//!

extern crate serde_json;
extern crate reqwest;

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
