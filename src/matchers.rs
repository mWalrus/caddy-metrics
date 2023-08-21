use prometheus_client::encoding::EncodeLabelValue;
use regex::Regex;

#[derive(Debug)]
pub struct Matcher {
    host: &'static str,
    uri_regex: Regex,
    method: Method,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelValue)]
pub enum Method {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
    UNKNOWN,
}

impl From<&str> for Method {
    fn from(value: &str) -> Self {
        match value {
            "GET" => Self::GET,
            "HEAD" => Self::HEAD,
            "POST" => Self::POST,
            "PUT" => Self::PUT,
            "DELETE" => Self::DELETE,
            "CONNECT" => Self::CONNECT,
            "OPTIONS" => Self::OPTIONS,
            "TRACE" => Self::TRACE,
            "PATCH" => Self::PATCH,
            _ => Self::UNKNOWN,
        }
    }
}

impl Matcher {
    pub fn new(host: &'static str, uri: &'static str, method: Method) -> Self {
        Self {
            host,
            uri_regex: Regex::new(uri.into()).unwrap(),
            method,
        }
    }

    pub fn is_match(&self, host: &str, uri: &str, method: &Method) -> bool {
        host == self.host && self.uri_regex.is_match(uri) && method == &self.method
    }

    pub fn matched_uri_segment(&self, uri: &str) -> String {
        if let Some(found) = self.uri_regex.find(uri) {
            String::from(uri.get(found.start()..found.end()).unwrap())
        } else {
            String::new()
        }
    }
}

pub fn init() -> Vec<Matcher> {
    vec![
        Matcher::new("tm.waalrus.xyz", "^/np/map/", Method::GET),
        Matcher::new("i.waalrus.xyz", "^/", Method::GET),
        Matcher::new("i.waalrus.xyz", "^/upload", Method::POST),
        Matcher::new("i.waalrus.xyz", "^/delete/", Method::DELETE),
        Matcher::new("searx.waalrus.xyz", "^/", Method::GET),
    ]
}
