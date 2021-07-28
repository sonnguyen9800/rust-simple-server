use crate::http::method::MethodError;
use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::str::Utf8Error;
use std::str;
use std::fmt::{Formatter, Result as FmtResult, Debug, Display};

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl Request {
    fn from_byte_array(buf: &[u8]) -> Result<Self, String> {
        unimplemented!()
    }
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {

        let request = str::from_utf8(buf)?;
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method : Method = method.parse()?;

        let mut query_string = None;

        if let Some(i) = path.find('?') {
            query_string = Some(path[i+1..].to_string());
            path = &path[..i];
        }

        Ok(Self {
            path: path.to_string(),
            query_string,
            method
        })

    }
}

fn get_next_word(request : &str) -> Option<(&str, &str)>{ 
    
    for (index,c) in request.chars().enumerate(){
        if c == ' ' || c == '\r' {
            return Some(
                (&request[..index], &request[index+1..])
            )    

        }
    }
    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}



impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method"

        }
    }
}
impl From<Utf8Error> for ParseError {
    fn from(_ : Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_ : MethodError) -> Self {
        Self::InvalidEncoding
    }
}
impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        writeln!(f, "{}", self.message())
    }
}
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        writeln!(f, "{}", self.message())
    }

}
impl Error for ParseError {

}