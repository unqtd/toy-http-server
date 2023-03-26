use crate::{http_error::HttpError, request::Request, response::Response};
use std::{
    io::{self, BufWriter, Write},
    net::TcpStream,
};

pub struct HttpConnection {
    stream: TcpStream,
}

impl HttpConnection {
    pub const fn new(stream: TcpStream) -> Self {
        Self { stream }
    }

    pub fn read_request(&mut self) -> Result<Request, HttpError> {
        let mut reqreader = request_reader::RequestReader::new(&mut self.stream);

        let (method, uri) = reqreader.read_starting_line()?;
        let headers = reqreader.read_headers()?;
        let body = headers
            .get_content_length()
            .and_then(|length| reqreader.read_body(length).ok());

        Ok(Request {
            method,
            uri,
            headers,
            body,
        })
    }

    pub fn send_response(&mut self, response: &Response) -> io::Result<()> {
        let mut bufwriter = BufWriter::new(&mut self.stream);

        write!(bufwriter, "HTTP/1.1 {}\r\n", response.code)?;
        write!(bufwriter, "{}\r\n", response.headers)?;
        bufwriter.write_all(&response.body)?;

        Ok(())
    }
}

mod request_reader {
    use crate::{
        http_error::HttpError,
        request::{headers::Headers, Method, Uri},
    };
    use std::{
        io::{self, BufRead, BufReader, Read},
        net::TcpStream,
    };

    pub struct RequestReader<'a> {
        bufreader: BufReader<&'a mut TcpStream>,
        buffer: String,
    }

    impl<'a> RequestReader<'a> {
        pub fn new(stream: &'a mut TcpStream) -> Self {
            Self {
                bufreader: BufReader::new(stream),
                buffer: String::with_capacity(100),
            }
        }

        pub fn read_starting_line(&mut self) -> Result<(Method, Uri), HttpError> {
            // Using self.buffer like buffer for reading starting line.
            self.bufreader
                .read_line(&mut self.buffer)
                .map_err(HttpError::Io)?;

            let (method, tail_of_line) = self
                .buffer
                .split_once(' ')
                .ok_or(HttpError::BadStartingLineSyntax)?;

            let (uri, _) = tail_of_line
                .split_once(' ')
                .ok_or(HttpError::BadStartingLineSyntax)?;

            let method_and_uri = (method.try_into()?, uri.to_owned());
            self.buffer.truncate(0);
            Ok(method_and_uri)
        }

        pub fn read_headers(&mut self) -> Result<Headers, HttpError> {
            let mut headers = Vec::new();

            loop {
                // Using self.buffer like buffer for reading header's line.
                self.bufreader
                    .read_line(&mut self.buffer)
                    .map_err(HttpError::Io)?;

                if self.buffer == "\r\n" {
                    break;
                }

                headers.push(parsers::parse_header(&self.buffer)?);
                self.buffer.truncate(0);
            }

            Ok(Headers(headers.into_boxed_slice()))
        }

        pub fn read_body(self, length: usize) -> Result<Box<[u8]>, io::Error> {
            let mut chunk = self.bufreader.take(length as u64);
            let mut body = Vec::with_capacity(length);
            chunk.read_to_end(&mut body)?;

            Ok(body.into_boxed_slice())
        }
    }

    mod parsers {
        use super::{HttpError, Method};

        impl<'a> TryFrom<&'a str> for Method {
            type Error = HttpError;

            fn try_from(value: &'a str) -> Result<Self, Self::Error> {
                Ok(match value {
                    "GET" => Self::Get,
                    "POST" => Self::Post,
                    _ => return Err(HttpError::UnknowMethod(value.to_owned())),
                })
            }
        }

        pub fn parse_header(line: &str) -> Result<(String, String), HttpError> {
            let (key, value) = line.split_once(':').ok_or(HttpError::BadHeaderSyntax)?;
            Ok((key.to_string(), value.trim().to_string()))
        }
    }
}

mod response_writers {
    use crate::response::Code;
    use std::fmt::Display;

    impl Display for Code {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Self::Ok => "200 OK",
                    Self::NotFound => "404 Not Found",
                }
            )
        }
    }
}
