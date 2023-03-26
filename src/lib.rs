mod connection;

pub mod http_error;
pub mod request;
pub mod response;

use connection::HttpConnection;
use http_error::HttpError;
use request::Request;
use response::Response;
use std::{
    io,
    net::{TcpListener, ToSocketAddrs},
};

pub struct ToyHttpServer<Handler> {
    listener: TcpListener,
    handler: Handler,
}

impl<Handler> ToyHttpServer<Handler>
where
    Handler: FnMut(Result<Request, HttpError>) -> Response,
{
    pub fn new<A: ToSocketAddrs>(addr: A, handler: Handler) -> io::Result<Self> {
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
