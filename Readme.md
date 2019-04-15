# exoplanet-archive
**WIP!** Small library for parsing data from [exoplanet archive API](https://exoplanetarchive.ipac.caltech.edu).
Currently there is only a very simple uncomplete low level api. In the future there will be a much
simpler High Level Api.

**NOTE:** As a work in progress not all API's have been implemented yet and everything
is subject to change.
## Requirements
  - This library uses [native-tls](https://crates.io/crates/native-tls) when fetching data, witch
   required openssl be installed.

## Usage
  - #### [Records](records)
     Records are the most basic representation of the exoplanet data. They
     implement the `ApiEndpoint` to retrieve data from the api.
     Here is an example of how to print the name of every 'Confirmed' exoplanet.
     ```rust
     extern crate exoplanet_archive;
 
     use exoplanet_archive::records::ConfirmedExoplanets;
     
     fn main() {
         let planets = ConfirmedExoplanetRecord::load().unwrap();
         for planet in planets {
            println!("{:?}", planet.pl_name);
         }
     }
     ```

     Select only specific data fields.
     ```rust
     extern crate exoplanet_archive;
 
     use exoplanet_archive::records::ConfirmedExoplanets;
     
     fn main() {
        let fields = "pl_hostname,pl_letter";
        ConfirmedExoplanetRecord::select(fields).unwrap();
         for planet in planets {
            println!("{:?}", planet.pl_name);
         }
     }
     ```
