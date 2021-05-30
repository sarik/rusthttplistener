use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
    GET,
    DELETE,
    POST,
    PATCH,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
}

/*impl TryFrom<&str>  for Method {

    fn try_from(val: &str) -> Result<Self, Self::Error> {


    }

}*/

//get Method from some string using somestring.parse()
impl FromStr for Method {
    type Err = MethodError;
    fn from_str(method: &str) -> Result<Self, Self::Err> {
        match method {
            "GET" => Ok(Self::GET),
            "DELETE" => Ok(Self::DELETE),
            "POST" => Ok(Self::POST),
            "PATCH" => Ok(Self::PATCH),
            "HEAD" => Ok(Self::HEAD),
            "CONNECT" => Ok(Self::CONNECT),
            "OPTIONS" => Ok(Self::OPTIONS),
            "TRACE" => Ok(Self::TRACE),
            _ => Err(MethodError),
        }
    }
}

pub struct MethodError ;