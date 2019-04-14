use serde_derive::{ Serialize, Deserialize };
use chrono::NaiveDate;
use crate::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfirmedExoplanet {
    pub pl_hostname: Option<String>,
    pub pl_letter: Option<String>,
    pub pl_name: Option<String>,
    pub pl_discmethod: Option<String>,
    pub pl_controvflag: Option<usize>,
    pub pl_pnum: Option<usize>,
    pub pl_orbper: Option<f64>,
    pub pl_orbpererr1: Option<f64>,
    pub pl_orbpererr2: Option<f64>,
    pub pl_orbperstr: Option<String>,
    pub pl_orbperlim: Option<f64>,
    pub pl_orbpern: Option<usize>,
    pub pl_orbsmax: Option<f64>,
    pub pl_orbsmaxerr1: Option<f64>,
    pub pl_orbsmaxerr2: Option<f64>,
    pub pl_orbsmaxstr: Option<String>,
    pub pl_orbsmaxlim: Option<f64>,
    pub pl_orbsmaxn: Option<f64>,
    pub pl_orbeccen: Option<f64>,
    pub pl_orbeccenerr1: Option<f64>,
    pub pl_orbeccenerr2: Option<f64>,
    pub pl_orbeccenstr: Option<String>,
    pub pl_orbeccenlim: Option<f64>,
    pub pl_orbincl: Option<f64>,
    pub pl_orbinclerr1: Option<f64>,
    pub pl_orbinclerr2: Option<f64>,
    pub pl_orbinclstr: Option<String>,
    pub pl_orbincllim: Option<f64>,
    pub pl_orbincln: Option<f64>,
    pub pl_bmassj: Option<f64>,
    pub pl_bmassjerr1: Option<f64>,
    pub pl_bmassjerr2: Option<f64>,
    pub pl_bmassjstr: Option<String>,
    pub pl_bmassjlim: Option<f64>,
    pub pl_bmassn: Option<f64>,
    pub pl_bmassprov: Option<String>,
    pub pl_radj: Option<f64>,
    pub pl_radjerr1: Option<f64>,
    pub pl_radjerr2: Option<f64>,
    pub pl_radjstr: Option<String>,
    pub pl_radjlim: Option<f64>,
    pub pl_radn: Option<f64>,
    pub pl_dens: Option<f64>,
    pub pl_denserr1: Option<f64>,
    pub pl_denserr2: Option<f64>,
    pub pl_densstr: Option<String>,
    pub pl_denslim: Option<f64>,
    pub pl_densm: Option<f64>,
    pub pl_ttvflag: Option<usize>,
    pub pl_kepflag: Option<usize>,
    pub pl_k2flag: Option<usize>,
    pub pl_nnotes: Option<usize>,
    pub ra_str: Option<String>,
    pub st_posn: Option<f64>,
    pub dec_str: Option<String>,
    pub ra: Option<f64>,
    pub dec: Option<f64>,
    pub st_dist: Option<f64>,
    pub st_disterr1: Option<f64>,
    pub st_disterr2: Option<f64>,
    pub st_diststr: Option<String>,
    pub st_distlim: Option<f64>,
    pub st_distn: Option<f64>,
    pub st_optmag: Option<f64>,
    pub st_optmagerr: Option<f64>,
    pub st_optmagstr: Option<String>,
    pub st_optmaglim: Option<f64>,
    pub st_optband: Option<String>,
    pub gaia_gmag: Option<f64>,
    pub gaia_gmagerr: Option<f64>,
    pub gaia_gmagstr: Option<String>,
    pub gaia_gmaglim: Option<f64>,
    pub st_teff: Option<f64>,
    pub st_tefferr1: Option<f64>,
    pub st_tefferr2: Option<f64>,
    pub st_teffstr: Option<String>,
    pub st_tefflim: Option<f64>,
    pub st_teffn: Option<f64>,
    pub st_mass: Option<f64>,
    pub st_masserr1: Option<f64>,
    pub st_masserr2: Option<f64>,
    pub st_massstr: Option<String>,
    pub st_masslim: Option<f64>,
    pub st_massn: Option<f64>,
    pub st_rad: Option<f64>,
    pub rowupdate: Option<NaiveDate>,
    pub pl_facility: Option<String>
}

impl ConfirmedExoplanet {

    pub fn get_url() -> String {
        "https://exoplanetarchive.ipac.caltech.edu/cgi-bin/nstedAPI/nph-nstedAPI?table=exoplanets&format=json".into()
    }

    pub fn load_all() -> Result<Vec<ConfirmedExoplanet>> {
        let data = reqwest::get(&Self::get_url())?.text()?;
        Ok(serde_json::from_str(&data)?)
    }
}

