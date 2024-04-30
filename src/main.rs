use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

async fn echo(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // Get the HTTP method, path and body from the request
    let (parts, body) = req.into_parts();

    // Read the request body as bytes
    let full_body = hyper::body::to_bytes(body).await.unwrap();

    // Construct the response with the same method, URI, headers and body
    let mut response = Response::new(Body::from(full_body));
    *response.headers_mut() = parts.headers.clone();
    *response.version_mut() = parts.version;

    Ok(response)
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(echo))
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}