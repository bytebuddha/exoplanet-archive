use serde_json::Error as JsonError;
use reqwest::Error as ReqwestError;
use std::io::Error as IoError;

#[derive(Debug)]
pub enum Error {
    Json(JsonError),
    Io(IoError),
    Http(ReqwestError)
}

macro_rules! impl_simple_from {
    ($type:ident, $variant:ident) => {
        impl From<$type> for Error {
            fn from(e: $type) -> Error {
                Error::$variant(e)
            }
        }
    }
}

impl_simple_from!(JsonError, Json);
impl_simple_from!(IoError, Io);
impl_simple_from!(ReqwestError, Http);
