use super::method::Method;
use std::convert::TryFrom;
use std::str;
use std::error::Error;
use std::fmt::{Formatter, Display, Debug, Result as FmtResult};

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl TryFrom<&[u8]> for Request{
    type Error = ParseError;


    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        match str::from_utf8(buf){
            Ok(request) => {},
            Err(e)=> return Err(ParseError::InvalidEncoding),
        }

        match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)){
            Ok(request) => {},
            Err(e)=> return Err(e);
        }
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidadMethodError,

}

impl Display for ParseError{
    fn fmt(&self, f: &mut Formatter)-> FmtResult {
        write!(f, "{}", self.message())
    }
}
impl Debug for ParseError{
    fn fmt(&self, f: &mut Formatter)-> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl ParseError {
    fn message(&self) -> &str{
        match self {
            Self::InvalidRequest => "InvalidRequest",
            Self::InvalidEncoding => "InvalidEncoding",
            Self::InvalidProtocol => "InvalidProtocol",
            Self::InvalidadMethodError => "InvalidadMethodError",
        }
    }
}

impl Error for ParseError {}