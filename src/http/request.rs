use std::convert::{From, TryFrom};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::from_utf8;
//use std::str::FromStr;
//use crate::str::from_utf8;
use super::method::{Method, MethodError};
use super::query_string::QueryString;

#[derive(Debug)]
pub struct Request<'a> {
    path: &'a str,
    query_string: Option<QueryString<'a>>,
    method: Method,
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl<'a> TryFrom<&'a [u8]> for Request<'a> {
    type Error = ParseError;

    //GET /somepath?name=sarik HTTP/1.1\r\n..Headers
    fn try_from(buf: &'a [u8]) -> Result<Request<'a>, Self::Error> {
        //fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        //let oktosend = Request{path:"avc".to_string(),query_string:Some("ac".to_string()),method:Method::POST};

        //Without question mark,res would be Res<&str,ParseError>
        let request = from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;

        let (method, request) = get_words_from_string(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) =
            get_words_from_string(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_words_from_string(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        //Implemented From<MethodError> for ParseError since the below line returns MethodError
        //and ? tries to convert it to ParseError as per function signature
        let method: Method = method.parse()?;

        let mut querystring = None;

        /* match path.find("?") {
            Some(i) => {
                //+1 to accomodate for 1 byte of ?
                querystring = Some(&path[i + 1..]);
                path = &path[..i];
            }
            _ => println!("no query parametes found"),
        }*/

        if let Some(i) = path.find("?") {
            //+1 to accomodate for 1 byte of ?
            querystring = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        let oktosend = Self {
            path: path,
            query_string: querystring,
            method: method,
        };
        return Ok(oktosend);

        //unimplemented!()
    }
}

fn get_words_from_string(req: &str) -> Option<(&str, &str)> {
    for (i, val) in req.chars().enumerate() {
        if val == ' ' || val == '\r' {
            return Some((&req[0..i], &req[i + 1..]));
        }
    }

    return None;
}

/*impl From<&[u8]> for Request {
    fn from(_: &[u8]) -> Self {
        unimplemented!()
    }
}*/

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.messgae())
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.messgae())
    }
}

impl ParseError {
    fn messgae(&self) -> &str {
        match self {
            Self::InvalidRequest => "InvalidRequest",
            Self::InvalidEncoding => "InvalidEncoding",
            Self::InvalidProtocol => "InvalidProtocol",
            Self::InvalidMethod => "InvalidMetho",
        }
    }
}

impl Error for ParseError {}
