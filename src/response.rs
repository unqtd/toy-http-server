#[derive(Debug)]
pub struct Response {
    pub(crate) code: Code,
    pub(crate) headers: String,
    pub(crate) body: Vec<u8>,
}

#[derive(Debug)]
pub enum Code {
    Ok,
    NotFound,
}

impl Response {
    #[must_use]
    pub const fn new(code: Code) -> Self {
        Self {
            code,
            headers: String::new(),
            body: Vec::new(),
        }
    }

    #[must_use]
    pub fn append_to_body<D: AsRef<[u8]>>(mut self, data: D) -> Self {
        self.body.extend_from_slice(data.as_ref());
        self
    }

    #[must_use]
    pub fn header<S: AsRef<str>>(mut self, key: S, value: S) -> Self {
        self.headers += &format!("{}: {}\r\n", key.as_ref(), value.as_ref());
        self
    }
}
