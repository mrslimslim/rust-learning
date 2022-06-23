/*
 * @Description:
 * @Version: 2.0
 * @Autor: tengyu
 * @Date: 2022-06-11 15:05:56
 * @LastEditors: tengyu
 * @LastEditTime: 2022-06-19 22:28:48
 */
use super::method::{Method, MethodError};
use super::{QueryString, QueryStringValue};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        // 还没写好可以用unimplemented宏
        // 方法1
        // match str::from_utf8(&buf) {
        //     Ok(request) => {}
        //     Err(_) => return Err(ParseError::InvalidEncoding),
        // };
        // 方法2
        // match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
        //     Ok(req) => {}
        //     Err(e) => return Err(e),
        // }

        // 方法3
        let req = str::from_utf8(buf)?;

        // match get_next_word(req) {
        //     Some((method, request)) => {}
        //     None => return Err(ParseError::InvalidRequest),
        // }
        let (method, req) = get_next_word(req).ok_or(ParseError::InvalidRequest)?;
        let (mut path, req) = get_next_word(req).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(req).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;
        let mut query_string = None;
        // match path.find("?") {
        //     Some(i) => {
        //         query_string = Some(&path[i + 1..]);
        //         path = &path[..i];
        //     }
        //     None => {}
        // }
        // 类似indexOf
        // let q = path.find("?");
        if let Some(i) = path.find("?") {
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

fn get_next_word(req: &str) -> Option<(&str, &str)> {
    for (i, c) in req.chars().enumerate() {
        if c == ' ' || c == '\r' {
            // Early return
            return Some((&req[..i], &req[i + 1..]));
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

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
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

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocal",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl Error for ParseError {}
