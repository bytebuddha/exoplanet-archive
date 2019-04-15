# exoplanets
**WIP!** Small library for parsing data from [exoplanet archive API](https://exoplanetarchive.ipac.caltech.edu).

## Requirements
  - This library uses [native-tsl](https://crates.io/crates/native-tls) when fetching data, witch
   required openssl be installed.

## Ussge
  - #### [Records](records)
     Records are the most basic representation of the exoplanet data. They
     implement the `ApiEndpoint` to retrieve data from the api.
     Here is an example of how to print the name of every 'Confirmed' exoplanet.
     ```rust
     extern crate exoplanets;
 
     use exoplanets::records::ConfirmedExoplanets;
     
     fn main() {
         let planets = ConfirmedExoplanetRecord::load().unwrap();
         for planet in planets {
            println!("{:?}", planet.pl_name);
         }
     }
     ```

     Select only specific data fields.
     ```rust
     extern crate exoplanets;
 
     use exoplanets::records::ConfirmedExoplanets;
     
     fn main() {
        let fields = "pl_name,pl_letter,pl_host";
        ConfirmedExoplanetRecord::select(fields).unwrap();
         for planet in planets {
            println!("{:?}", planet.pl_name);
         }
     }
     ```
