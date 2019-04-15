//! All confirmed planets (and hosts) in the archive with parameters
//! derived from a single, published reference that are designated as
//! the archive's default parameter set.

use serde_derive::{ Serialize, Deserialize };
use chrono::NaiveDate;
use crate::Result;
use crate::records::ApiEndpoint;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfirmedExoplanetRecord {

    /// #### Host star name.
    ///
    /// Stellar name most commonly used in the literature.
    pub pl_hostname: Option<String>,

    /// #### Planet Letter
    ///
    /// Letter assigned to the planetary component of a planetary system.
    pub pl_letter: Option<String>,

    /// #### Planet Name
    ///
    /// Planet name most commonly used in the literature.
    pub pl_name: Option<String>,

    /// #### Discovery Method
    ///
    /// Method by which the planet was first identified.
    pub pl_discmethod: Option<String>,

    /// #### Controversial Flag
    ///
    /// Flag indicating whether the confirmation status of a planet has been
    /// questioned in the published literature (1=yes, 0=no)
    pub pl_controvflag: Option<usize>,

    /// #### Number of planets in System
    ///
    /// Number of planets in the planetary system.
    pub pl_pnum: Option<usize>,

    /// #### Orbital Period (days)
    ///
    /// Time the planet takes to make a complete orbit around
    /// the host star or system.
    pub pl_orbper: Option<f64>,

    /// #### Orbital Period Positive Uncertainty.
    pub pl_orbpererr1: Option<f64>,

    /// #### Orbital Period Negative Uncertainty.
    pub pl_orbpererr2: Option<f64>,

    /// #### Orbital Period Uncertainty Display String.
    pub pl_orbperstr: Option<String>,

    /// #### Orbital Period Limit Number.
    pub pl_orbperlim: Option<f64>,

    /// #### Orbital Period Number of Measurements
    pub pl_orbpern: Option<usize>,

    /// #### Orbit Semi-Major Axis (AU)
    ///
    /// The longest radius of an elliptic orbit, or, for exoplanets
    /// detected via gravitational microlensing or direct imaging,
    /// the projected separation in the plane of the sky.
    pub pl_orbsmax: Option<f64>,

    /// #### Orbit Semi-Major Axis Positive Uncertainty.
    pub pl_orbsmaxerr1: Option<f64>,

    /// #### Orbit Semi-Major Axis Negative Uncertainty.
    pub pl_orbsmaxerr2: Option<f64>,

    /// #### Orbit Semi-Major Axis Display String.
    pub pl_orbsmaxstr: Option<String>,

    /// #### Orbit Semi-Major Axis Limit Number.
    pub pl_orbsmaxlim: Option<f64>,

    /// #### Orbit Semi-Major Axis Number Of Measurements.
    pub pl_orbsmaxn: Option<f64>,

    /// #### Eccentricity
    ///
    /// Amount by which the orbit of the planet deviates from a perfect circle.
    pub pl_orbeccen: Option<f64>,

    /// #### Eccentricity Positive Uncertainty.
    pub pl_orbeccenerr1: Option<f64>,

    /// #### Eccentricity Negative Uncertainty.
    pub pl_orbeccenerr2: Option<f64>,

    /// #### Eccentricity Display String.
    pub pl_orbeccenstr: Option<String>,

    /// #### Eccentricity Limit Number.
    pub pl_orbeccenlim: Option<f64>,

    /// #### Eccentricity Number Of Measurements.
    pub pl_orbeccenn: Option<f64>,

    /// #### Inclination (deg).
    ///
    /// Angular distance of the orbital plane from the line of sight.
    pub pl_orbincl: Option<f64>,

    /// #### Inclination Positive Uncertainty.
    pub pl_orbinclerr1: Option<f64>,

    /// #### Inclination Negative Uncertainty.
    pub pl_orbinclerr2: Option<f64>,

    /// #### Inclination Display String.
    pub pl_orbinclstr: Option<String>,

    /// #### Inclination Limit Number.
    pub pl_orbincllim: Option<f64>,

    /// #### Inclination Number Of Measurements.
    pub pl_orbincln: Option<f64>,

    /// #### Planet Mass or M*sin(i) [Jupiter mass]
    ///
    /// Best planet mass estimate available, in order of preference:
    /// Mass, M*sin(i)/sin(i), or M*sin(i), depending on availability,
    /// and measured in Jupiter masses. See Planet Mass M*sin(i)
    /// Provenance (pl_bmassprov) to determine which measure applies.
    pub pl_bmassj: Option<f64>,

    /// #### Planet Mass Positive Uncertainty.
    pub pl_bmassjerr1: Option<f64>,

    /// #### Planet Mass Negative Uncertainty.
    pub pl_bmassjerr2: Option<f64>,

    /// #### Planet Mass Display String.
    pub pl_bmassjstr: Option<String>,

    /// #### Planet Mass Limit Number.
    pub pl_bmassjlim: Option<f64>,

    /// #### Planet Mass Number Of Measurements.
    pub pl_bmassn: Option<f64>,

    /// #### Planet Mass or M*sin(i) Provenance
    ///
    /// Provenance of the measurement of the best mass.
    /// Options are: Mass, M*sin(i)/sin(i), and M*sini.
    pub pl_bmassprov: Option<String>,

    /// #### Planet Radius (Jupiter radii)
    ///
    /// Length of a line segment from the center of the planet to its surface,
    /// measured in units of radius of Jupiter.
    pub pl_radj: Option<f64>,

    /// #### Planet Radius Positive Uncertainty.
    pub pl_radjerr1: Option<f64>,

    /// #### Planet Mass Negative Uncertainty.
    pub pl_radjerr2: Option<f64>,

    /// #### Planet Mass Display String.
    pub pl_radjstr: Option<String>,

    /// #### Planet Mass Limit Number.
    pub pl_radjlim: Option<f64>,

    /// #### Planet Mass Number Of Measurements.
    pub pl_radn: Option<f64>,

    /// #### Planet Density (g/cm**3)
    ///
    /// Amount of mass per unit of volume of the planet.
    pub pl_dens: Option<f64>,

    /// #### Planet Density Positive Uncertainty.
    pub pl_denserr1: Option<f64>,

    /// #### Planet Density Negative Uncertainty.
    pub pl_denserr2: Option<f64>,

    /// #### Planet Density Displya String.
    pub pl_densstr: Option<String>,

    /// #### Planet Density Limit Number.
    pub pl_denslim: Option<f64>,

    /// #### Planet Density Number Of Measurements.
    pub pl_densm: Option<f64>,

    /// #### TTV Flag
    ///
    /// Flag indicating if the planet orbit exhibits transit timing variationsfrom
    /// another planet in the system (1=yes, 0=no).
    ///
    /// Note: Non-transiting planets discovered via the transit timing variations of
    /// another planet in the system will not have their TTV flag set, since they do
    /// not themselves demonstrate TTVs.
    pub pl_ttvflag: Option<usize>,

    /// #### Kepler Field Flag
    ///
    /// Flag indicating if the planetary system signature is present in
    /// data taken with the Kepler mission (1=yes, 0=no).
    pub pl_kepflag: Option<usize>,

    /// #### K2 Mission Flag
    ///
    /// Flag indicating if the planetary system signature is
    /// present in data taken with the K2 Mission (1=yes, 0=no).
    pub pl_k2flag: Option<usize>,

    /// #### Number of Notes
    ///
    /// Number of Notes associated with the planet.
    /// View all notes in the Confirmed Planet Overview page.
    pub pl_nnotes: Option<usize>,

    /// #### RA (sexagesimal)
    ///
    /// Right Ascension of the planetary system in sexagesimal format.
    pub ra_str: Option<String>,

    /// #### RA (sexagesimal) Number Of Meansurements
    pub st_posn: Option<f64>,

    /// #### Dec (sexagesimal)
    ///
    /// Declination of the planetary system in sexagesimal notation.
    pub dec_str: Option<String>,

    /// #### RA (decimal degrees)
    ///
    /// Right Ascension of the planetary system in decimal degrees.
    pub ra: Option<f64>,

    /// #### Dec (decimal degrees)
    ///
    /// Declination of the planetary system in decimal degrees.
    pub dec: Option<f64>,

    /// #### Distance (pc)
    ///
    /// Distance to the planetary system in units of parsecs.
    pub st_dist: Option<f64>,

    /// #### Distance (pc) Positive Uncertainty.
    pub st_disterr1: Option<f64>,

    /// #### Distance (pc) Negative Uncertainty.
    pub st_disterr2: Option<f64>,

    /// #### Distance (pc) Display String.
    pub st_diststr: Option<String>,

    /// #### Distance (pc) Limit Number.
    pub st_distlim: Option<f64>,

    /// #### Distance (pc) Number Of Measurements.
    pub st_distn: Option<f64>,

    /// #### Optical Magnitude [mag]
    ///
    /// Brightness of the host star as measured using the
    /// V (Johnson) or the Kepler-band in units of magnitudes.
    pub st_optmag: Option<f64>,

    /// #### Optical Magnitude Uncertainty.
    pub st_optmagerr: Option<f64>,

    /// #### Optical Magnitude Display String.
    pub st_optmagstr: Option<String>,

    /// #### Optical Magnitude Limit Number.
    pub st_optmaglim: Option<f64>,

    /// #### Optical Magnitude Band
    ///
    /// Band corresponding to the Optical Magnitude. Options are: V (Johnson) or Kepler-band.
    pub st_optband: Option<String>,

    /// #### G-band (Gaia) [mag]
    ///
    /// Brightness of the host star as measuring using
    /// the Gaia band in units of magnitudes.
    /// Objects matched to Gaia using the Hipparcos or 2MASS IDs provided in Gaia DR2.
    pub gaia_gmag: Option<f64>,

    /// #### G-band (Gaia) Uncertainty.
    pub gaia_gmagerr: Option<f64>,

    /// #### G-band (Gaia) Display String.
    pub gaia_gmagstr: Option<String>,

    /// #### G-band (Gaia) Limit Number.
    pub gaia_gmaglim: Option<f64>,

    /// #### Effective Temperature (K)
    ///
    /// Temperature of the star as modeled by a
    /// black body emitting the same total amount of electromagnetic radiation.
    pub st_teff: Option<f64>,

    /// #### Effective Temperature Positive Uncertainty.
    pub st_tefferr1: Option<f64>,

    /// #### Effective Temperature Negative Uncertainty.
    pub st_tefferr2: Option<f64>,

    /// #### Effective Temperature Display String.
    pub st_teffstr: Option<String>,

    /// #### Effective Temperature Limit Number.
    pub st_tefflim: Option<f64>,

    /// #### Effective Temperature Number of Measurements.
    pub st_teffn: Option<f64>,

    /// #### Stellar Mass (solar mass)
    ///
    /// Amount of matter contained in the star, measured in units of masses of the Sun.
    pub st_mass: Option<f64>,

    /// #### Steller Mass Positive Uncertainty.
    pub st_masserr1: Option<f64>,

    /// #### Steller Mass Negative Uncertainty.
    pub st_masserr2: Option<f64>,

    /// #### Steller Mass Display String.
    pub st_massstr: Option<String>,

    /// #### Steller Mass Limit Number.
    pub st_masslim: Option<f64>,

    /// #### Steller Mass Number Of Measurements.
    pub st_massn: Option<f64>,

    /// #### Stellar Radius (solar radii)
    ///
    /// Length of a line segment from the center of the star to its surface,
    /// measured in units of radius of the Sun.
    pub st_rad: Option<f64>,

    /// #### Steller Radius (solar mass) Positive Uncertainty.
    pub st_raderr1: Option<f64>,

    /// #### Steller Radius (solar mass) Negative Uncertainty.
    pub st_raderr2: Option<f64>,

    /// #### Steller Radius Display String.
    pub st_radstr: Option<String>,

    /// #### Steller Radius Limit Number.
    pub st_radlim: Option<f64>,

    /// #### Steller Radius Number Of Measurements.
    pub st_radn: Option<f64>,

    /// #### Date of Last Update
    ///
    /// Date of last update of the planet parameters.
    pub rowupdate: Option<NaiveDate>,

    /// #### Discovery Facility
    ///
    /// Name of facility of planet discovery observations
    pub pl_facility: Option<String>
}

impl ApiEndpoint for ConfirmedExoplanetRecord {

    const TABLE_NAME: &'static str = "exoplanets";

    fn handle_data(data: &str) -> Result<Vec<Self>> {
        Ok(serde_json::from_str(&data)?)
    }
}
