use std::fmt::{Display, Formatter, Error};

pub enum EloGetError {
    HttpError,
    JsonError,
    InvalidSource,
}

impl Display for EloGetError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            EloGetError::HttpError => write!(f, "Error while handling http stuff."),
            EloGetError::JsonError => write!(f, "Error while parsing json."),
            EloGetError::InvalidSource => write!(f, "Specified source is not valid, or is mispelled."),
        }
    }
}

impl From<ureq::Error> for EloGetError {
    fn from(err: ureq::Error) -> Self {
        EloGetError::HttpError
    }
}

