use toy_http_server::{
    response::{Code, Response},
    HttpRequest, ToyHttpServer,
};

fn main() {
    ToyHttpServer::new("localhost:7000", handler)
        .unwrap()
        .serve()
}

fn handler(request: HttpRequest) -> Response {
    println!("{request:?}");

    Response::new(Code::Ok)
        .header("Content-Type", "text/html")
        .append_to_body(b"<h1>Hello World!</h1>")
}
