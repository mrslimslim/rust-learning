/*
 * @Description:
 * @Version: 2.0
 * @Autor: tengyu
 * @Date: 2022-06-11 15:06:02
 * @LastEditors: tengyu
 * @LastEditTime: 2022-06-11 23:02:13
 */

use std::str::FromStr;

pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    OPTIONS,
    PATCH,
}

impl FromStr for Method {
    type Err = MethodError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "DELETE" => Ok(Self::DELETE),
            "OPTIONS" => Ok(Self::OPTIONS),
            "PATCH" => Ok(Self::PATCH),
            _ => Err(MethodError),
        }
    }
}

pub struct MethodError;
