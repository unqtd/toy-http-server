mod connection;

pub mod http_error;
pub mod request;
pub mod response;

use anyhow::Result;
use connection::HttpConnection;
use request::Request;
use response::Response;
use std::net::{TcpListener, ToSocketAddrs};

pub struct ToyHttpServer<Handler> {
    listener: TcpListener,
    handler: Handler,
}

impl<Handler> ToyHttpServer<Handler>
where
    Handler: FnMut(Result<Request>) -> Response,
{
    pub fn new<A: ToSocketAddrs>(addr: A, handler: Handler) -> Result<Self> {
        Ok(Self {
            listener: TcpListener::bind(addr)?,
            handler,
        })
    }

    pub fn serve(mut self) -> ! {
        for stream in self.listener.incoming() {
            let mut connection = HttpConnection::new(stream.unwrap());

            let response = (self.handler)(connection.get());
            connection.send(response).unwrap();
        }
        unreachable!()
    }
}
