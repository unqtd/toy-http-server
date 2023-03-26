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

pub type HttpRequest = Result<Request, HttpError>;

pub struct ToyHttpServer<Handler> {
    listener: TcpListener,
    handler: Handler,
}

impl<Handler> ToyHttpServer<Handler>
where
    Handler: FnMut(HttpRequest) -> Response,
{
    /// # Errors
    /// Returns an error if it is impossible to bind `TcpListener` with the passed address.
    pub fn new<A: ToSocketAddrs>(addr: A, handler: Handler) -> io::Result<Self> {
        Ok(Self {
            listener: TcpListener::bind(addr)?,
            handler,
        })
    }

    pub fn serve(mut self) -> ! {
        for stream in self.listener.incoming() {
            let mut connection =
                HttpConnection::new(stream.expect("Failed to get TCP stream connection."));

            let response = (self.handler)(connection.read_request());
            connection
                .send_response(&response)
                .expect("Failed on trying to send response.");
        }
        unreachable!()
    }
}
