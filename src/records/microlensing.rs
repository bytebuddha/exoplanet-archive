use serde_derive::{ Serialize, Deserialize };

use crate::Result;
use crate::records::ApiEndpoint;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MicrolensingExoplanetRecord {
    pub plntname: Option<String>,
    pub ra_str: Option<String>,
    pub dec_str: Option<String>,
    pub mlmassplnj: Option<f64>,
    pub mlmassplnjerr1: Option<f64>,
    pub mlmassplnjerr2: Option<f64>,
    pub mlmassplnjlim: Option<f64>,
    pub mlmassplne: Option<f64>,
    pub mlmassplneerr1:Option<f64>,
    pub mlmassplneerr2:Option<f64>,
    pub mlmassplnelim:Option<f64>,
    pub mlsmaproj: Option<f64>,
    pub mlsmaprojerr1: Option<f64>,
    pub mlsmaprojerr2: Option<f64>,
    pub mlsmaprojlim: Option<f64>,
    pub mlmasslens: Option<f64>,
    pub mlmasslenserr1: Option<f64>,
    pub mlmasslenserr2: Option<f64>,
    pub mlmasslenslim: Option<f64>,
    pub mldistl: Option<f64>,
    pub mldistlerr1: Option<f64>,
    pub mldistlerr2: Option<f64>,
    pub mldistllim: Option<f64>,
    pub mldists: Option<f64>,
    pub mldistserr1: Option<f64>,
    pub mldistserr2: Option<f64>,
    pub mldistslim: Option<f64>,
    pub mltsepmin: Option<f64>,
    pub mltsepminerr1: Option<f64>,
    pub mltsepminerr2: Option<f64>,
    pub mltsepminlim: Option<f64>,
    pub mlsepminnorm: Option<f64>,
    pub mlsepminnormerr1: Option<f64>,
    pub mlsepminnormerr2: Option<f64>,
    pub mlsepminnormlim: Option<f64>,
    pub mlxtimeein: Option<f64>,
    pub mlxtimeeinerr1: Option<f64>,
    pub mlxtimeeinerr2: Option<f64>,
    pub mlxtimeeinlim: Option<f64>,
    pub mlradsnorm: Option<f64>,
    pub mlradsnormerr1: Option<f64>,
    pub mlradsnormerr2: Option<f64>,
    pub mlradsnormlim: Option<f64>,
    pub mlsepinsnorp: Option<f64>,
    pub mlsepinsnorperr1: Option<f64>,
    pub mlsepinsnorperr2: Option<f64>,
    pub mlsepinsnorplim: Option<f64>,
    pub mlmassratio: Option<f64>,
    pub mlmassratioerr1: Option<f64>,
    pub mlmassratioerr2: Option<f64>,
    pub mlangstlax: Option<f64>,
    pub mlangstlaxerr1: Option<f64>,
    pub mlangstlaxerr2: Option<f64>,
    pub mlangstlaxlim: Option<f64>,
    pub mlmagis: Option<f64>,
    pub mlmagiserr1: Option<f64>,
    pub mlmagiserr2: Option<f64>,
    pub mlmagislim: Option<f64>,
    pub mlmagibl: Option<f64>,
    pub mlmagiblerr1: Option<f64>,
    pub mlmagiblerr2: Option<f64>,
    pub mlmagibllim: Option<f64>,
    pub mlradeinang: Option<f64>,
    pub mlradeinangerr1: Option<f64>,
    pub mlradeinangerr2: Option<f64>,
    pub mlradeinanglim: Option<f64>,
    pub mlpmrells: Option<f64>,
    pub mlpmrellserr1: Option<f64>,
    pub mlpmrellserr2: Option<f64>,
    pub mlpmrellslim: Option<f64>,
    pub mlmodeldef: Option<f64>,
    pub plntreflink: Option<String>
}

impl ApiEndpoint for MicrolensingExoplanet {

    const TABLE_NAME: &'static str = "microlensing";

    fn handle_data(data: &str) -> Result<Vec<Self>> {
        Ok(serde_json::from_str(&data)?)
    }
}