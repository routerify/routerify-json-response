use hyper::{Body as HyperBody, Request, Response, Server, StatusCode};
use json_response::{json_failed_resp, json_failed_resp_with_message, json_success_resp, json_success_resp_with_code};
use routerify::{Router, RouterService};
use std::{convert::Infallible, net::SocketAddr};
use stream_body::StreamBody;

async fn home_handler(_: Request<HyperBody>) -> Result<Response<StreamBody>, Infallible> {
    Ok(json_success_resp_with_code(StatusCode::ACCEPTED, &["Alice", "John"]).unwrap())
}

fn router() -> Router<StreamBody, Infallible> {
    Router::builder().get("/", home_handler).build().unwrap()
}

#[tokio::main]
async fn main() {
    let router = router();

    let service = RouterService::new(router);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    let server = Server::bind(&addr).serve(service);

    println!("App is running on: {}", addr);
    if let Err(err) = server.await {
        eprintln!("Server error: {}", err);
    }
}
