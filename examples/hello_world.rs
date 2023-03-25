use toy_http_server::{
    response::{Code, Response},
    ToyHttpServer,
};

fn main() {
    let server = ToyHttpServer::new("localhost:7000", |req| {
        println!("Req: {req:?}");

        Response::new(Code::Ok)
            .header("Content-Type", "text/html")
            .append_to_body("<h1>Hello World!</h1>".as_bytes())
    })
    .unwrap();

    server.serve()
}
