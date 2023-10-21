use std::error::Error;
use std::convert::TryFrom;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::{from_utf8, Utf8Error};
use crate::http::method::MethodError;
use crate::http::query_string::QueryString;
use super::method::Method;

#[derive(Debug)]
pub struct Request<'buf> {
    pub(crate) path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}


impl<'buf> Request<'buf> {
    fn from_byte_array(buf: &[u8]) {}
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method {
     &self.method
}
    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }

}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let request = from_utf8(buf)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        /*        let result: Result<(&str, &str), ParseError> = get_next_word(request).ok_or(ParseError::InvalidRequest);
                let y = result?;*/

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }
        let method: Method = method.parse()?;

        let mut query_string = None;
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}


/*fn testme() {
    let mystring = String::from("Hello world");
    let stringref = get_next_word2(&mystring);
}

fn get_next_word2(request: &str) -> &str {
    let mystring: &str = &String::from("Hello world");

    return mystring;
}
*/

 fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, char) in request.chars().enumerate() {
        if char == ' ' || char == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    return None;
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
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}