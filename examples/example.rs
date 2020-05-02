use hyper::{Body, Request, Response, Server, StatusCode};
// Import required json_response methods.
use json_response::{json_failed_resp_with_message, json_success_resp};
use routerify::{Router, RouterService};
use std::net::SocketAddr;

async fn list_users_handler(_: Request<Body>) -> Result<Response<Body>, routerify::Error> {
    // Fetch response data from somewhere.
    let users = ["Alice", "John"];

    // Generate a success JSON response with the data in the following format:
    // { "status": "success", code: 200, data: ["Alice", "John"] }
    json_success_resp(&users)
}

async fn list_books_handler(_: Request<Body>) -> Result<Response<Body>, routerify::Error> {
    // Generate a failed JSON response in the following format:
    // { "status": "failed", code: 500, data: "Internal Server Error: Couldn't fetch book list from database" }
    json_failed_resp_with_message(
        StatusCode::INTERNAL_SERVER_ERROR,
        "Couldn't fetch book list from database",
    )
}

// Create a router.
fn router() -> Router<Body, routerify::Error> {
    Router::builder()
        // Attach the handlers.
        .get("/users", list_users_handler)
        .get("/books", list_books_handler)
        .build()
        .unwrap()
}

#[tokio::main]
async fn main() {
    let router = router();

    // Create a Service from the router above to handle incoming requests.
    let service = RouterService::new(router);

    // The address on which the server will be listening.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    // Create a server by passing the created service to `.serve` method.
    let server = Server::bind(&addr).serve(service);

    println!("App is running on: {}", addr);
    if let Err(err) = server.await {
        eprintln!("Server error: {}", err);
    }
}
