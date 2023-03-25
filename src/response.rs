pub struct Response {
    pub(crate) code: Code,
    pub(crate) headers: String,
    pub(crate) body: Vec<u8>,
}

pub enum Code {
    Ok,
    NotFound,
}

impl Response {
    pub fn new(code: Code) -> Self {
        Self {
            code,
            headers: String::new(),
            body: Vec::new(),
        }
    }

    pub fn append_to_body(mut self, data: &[u8]) -> Self {
        self.body.extend_from_slice(data);
        self
    }

    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.headers += &format!("{key}: {value}\r\n");
        self
    }
}
