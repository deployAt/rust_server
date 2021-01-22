use std::str::FromStr;

pub enum Method {
  GET,
  DELTE,
  POST,
  PUT,
  HEAD,
  CONNECT,
  OPTIONS,
  TRACE,
  PATCH,
}

impl FromStr for Method {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "GET" => Ok(Self::GET),
      "DELTE" => Ok(Self::DELTE),
      "POST" => Ok(Self::POST),
      "PUT" => Ok(Self::PUT),
      "HEAD" => Ok(Self::HEAD),
      "CONNECT" => Ok(Self::CONNECT),
      "OPTIONS" => Ok(Self::OPTIONS),
      "TRACE" => Ok(Self::TRACE),
      "PATCH" => Ok(Self::PATCH),
      _ => Err(String::from(MethodErorr)),
    }
  }
}

pub struct MethodErorr;
