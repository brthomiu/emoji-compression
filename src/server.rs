use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

// Hello world function
async fn hello(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let response_body = format!("Rust HTTP server with Hyper! Running on port 3000");
    Ok(Response::new(Body::from(response_body)))
}

pub async fn start_server() {
    // Bind to 127.0.0.1:port
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Service for hello
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(hello)) });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Server running on port 3000.");

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
