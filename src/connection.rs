use crate::{request::Request, response::Response};
use anyhow::Result;
use std::{
    io::{BufWriter, Write},
    net::TcpStream,
};

pub struct HttpConnection {
    stream: TcpStream,
}

impl HttpConnection {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }

    pub fn get(&mut self) -> Result<Request> {
        let mut reqreader = request_reader::RequestReader::new(&mut self.stream);

        let (method, uri) = reqreader.read_starting_line()?;

        let headers = reqreader.read_headers()?;

        let body =
            if let Some((_, length)) = headers.iter().find(|(key, _)| key == "Content-Length") {
                Some(reqreader.read_body(length.parse()?)?)
            } else {
                None
            };

        Ok(Request {
            method,
            uri,
            headers,
            body,
        })
    }

    pub fn send(&mut self, response: Response) -> Result<()> {
        let mut bufwriter = BufWriter::new(&mut self.stream);

        write!(bufwriter, "HTTP/1.1 {}\r\n", response.code)?;
        write!(bufwriter, "{}\r\n", response.headers)?;
        bufwriter.write_all(&response.body)?;

        Ok(())
    }
}

mod request_reader {
    use std::{
        io::{BufRead, BufReader, Read},
        net::TcpStream,
    };

    use crate::{
        http_error::HttpError,
        request::{Headers, Method, Uri},
    };
    use anyhow::{Context, Result};

    pub struct RequestReader<'a> {
        bufreader: BufReader<&'a mut TcpStream>,
    }

    impl<'a> RequestReader<'a> {
        pub fn new(stream: &'a mut TcpStream) -> Self {
            Self {
                bufreader: BufReader::new(stream),
            }
        }

        pub fn read_starting_line(&mut self) -> Result<(Method, Uri)> {
            let mut starting_line = String::with_capacity(20);
            self.bufreader
                .read_line(&mut starting_line)
                .context("Failed to read starting_line from TCP stream.")?;

            let (method, tail_of_line) = starting_line
                .split_once(' ')
                .ok_or(HttpError::BadStartingLineSyntax)?;

            let (uri, _) = tail_of_line
                .split_once(' ')
                .ok_or(HttpError::BadStartingLineSyntax)?;

            Ok((method.try_into()?, uri.to_owned()))
        }

        pub fn read_headers(&mut self) -> Result<Headers> {
            let mut headers = Headers::new();

            let mut buffline = String::with_capacity(20);
            loop {
                self.bufreader
                    .read_line(&mut buffline)
                    .context("Failed to read line of header.")?;

                if buffline == "\r\n" {
                    break;
                }

                headers.push(parsers::parse_header(&buffline)?);
                buffline.truncate(0);
            }

            Ok(headers)
        }

        /// # Warnings
        /// On 32-bit (or other platforms other than amd64) platforms, reading the
        /// request body that is larger than `u32::MAX` will lead to undefined behavior!
        pub fn read_body(self, length: u64) -> Result<Box<[u8]>> {
            let mut chunk = self.bufreader.take(length);
            let mut body = Vec::with_capacity(length as usize);
            chunk
                .read_to_end(&mut body)
                .with_context(|| format!("Failed to read_exact body with length {length}."))?;

            Ok(body.into_boxed_slice())
        }
    }

    mod parsers {
        use super::*;

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

        pub fn parse_header(line: &str) -> Result<(String, String)> {
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
