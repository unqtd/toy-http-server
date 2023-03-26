#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub uri: Uri,
    pub headers: headers::Headers,
    pub body: Option<Box<[u8]>>,
}

#[derive(Debug)]
pub enum Method {
    Get,
    Post,
}

pub type Uri = String;

pub mod headers {
    #[derive(Debug)]
    pub struct Headers(pub(crate) Box<[(String, String)]>);

    impl Headers {
        #[must_use]
        pub fn get_content_length(&self) -> Option<usize> {
            self.0
                .iter()
                .find(|(key, _)| key == "Content-Length")
                .and_then(|(_, length)| length.parse().ok())
        }
    }
}
