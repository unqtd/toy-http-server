#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub uri: Uri,
    pub headers: Headers,
    pub body: Option<Box<[u8]>>,
}

#[derive(Debug)]
pub enum Method {
    Get,
    Post,
}

pub type Uri = String;
pub type Headers = Box<[(String, String)]>;
