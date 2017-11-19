use std::fmt;
use std::str;

#[derive(Debug, Copy, Clone)]
pub enum HttpVersion {
    Http10,
    Http11,
    INVALID
}


#[derive(Debug, Copy, Clone)]
pub enum HttpMethod {
    GET,
//    POST,
//    PUT,
    INVALID
}


pub struct Request {
    method : HttpMethod,
    resource : String,
    version : HttpVersion
}

fn parse_version (version : &str) -> HttpVersion
{
    if version == "HTTP/1.1" {
        return HttpVersion::Http11;
    }
    if version ==  "HTTP/1.0" {
        print!("HTTP1");
        return HttpVersion::Http10;
    }

    return HttpVersion::INVALID;
}


fn parse_method (method : &str) -> HttpMethod
{
    if method == "GET" {
        print!("match!");
        return HttpMethod::GET;
    }
    return HttpMethod::INVALID;
}


impl Request {
    pub fn new(m : &str, r : &str, v : &str) -> Self {
        let method_parsed = parse_method(m);
        let version_parsed = parse_version(v);

        Request {method  : method_parsed,
                resource : String::from(r),
                version  : version_parsed}
    }


    pub fn get_method(&self) -> HttpMethod {
        self.method.clone()
    }

    pub fn get_resource(&self) -> &String {
        &self.resource
    }

}

impl fmt::Debug for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Method {:?}, resource: {}, http version {:?}", self.method, self.resource, self.version)
    }
}

pub struct Reply {
    version : HttpVersion,
    pub content : Vec<u8>,
    code : u16,
}

impl Reply {
    pub fn new_from_request(r : &Request) -> Self {
        let ver : HttpVersion = (*r).version.clone();
        let rep = Reply {version : ver,
                         content : Vec::new(),
                         code : 0};
        return rep;

    }

    pub fn set_code (&mut self, c : u16) {
        self.code = c;
    }

    pub fn get_preamble(&self) -> String {
        let mut s = String::new();
        match self.version
        {
            HttpVersion::Http10 =>
            {
                s.push_str("HTTP/1.0");
            },
            HttpVersion::Http11 =>
            {
                s.push_str("HTTP/1.1");
            },
            _ =>
            {
                s.push_str("HTTP/1.0");
            }
        }

        match self.code
        {
            404 =>
            {
                s.push_str(" 404 Not Found\n\r\nNot found");
            },
            200 =>
            {
                s.push_str(" 200 OK\n\r\n");
            },
            _ =>
            {
                s.push_str(" NOT IMPLEMENTED");
            }
        }

        return s;
    }

}


