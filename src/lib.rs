//! # exoplanet
//!
//! Library for parsing data from [exoplanet archive API](https://exoplanetarchive.ipac.caltech.edu)
//!
//! ## Requirements
//! - This library uses [native-tsl](https://crates.io/crates/native-tls) when fetching data, witch
//!   required openssl be installed.
//!
//! ## Ussge
//! - #### [Records](records)
//!     Records are the most basic representation of the exoplanet data. They
//!     implement the `ApiEndpoint` to retrieve data from the api.
//!     ```rust
//!         use exoplanets::records::ApiEndpoint;
//!         use exoplanets::records::ConfirmedExoplanetRecord;
//!
//!         let planets = ConfirmedExoplanetRecord::load().unwrap();
//!         for planet in planets {
//!            println!("{:?}", planet.pl_name);
//!         }
//!     ```
//!
//!     ##### Select
//!     You can select the fields you would like to retrieve from the api using the select method.
//!     It takes a comma seperated list of field names to retrieve from the server.
//!     ```rust
//!        use exoplanets::records::ApiEndpoint;
//!        use exoplanets::records::ConfirmedExoplanetRecord;
//!
//!        let fields = "pl_name,pl_letter,pl_hostname";
//!        ConfirmedExoplanetRecord::select(fields).unwrap();
//!     ```
//!

extern crate serde_json;
extern crate reqwest;

mod error;
pub use self::error::Error;

pub type Result<T> = ::std::result::Result<T, Error>;

pub mod records;
