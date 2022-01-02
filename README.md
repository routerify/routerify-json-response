[![Github Actions Status](https://github.com/routerify/routerify-json-response/workflows/Test/badge.svg)](https://github.com/routerify/routerify-json-response/actions)
[![crates.io](https://img.shields.io/crates/v/routerify-json-response.svg)](https://crates.io/crates/routerify-json-response)
[![Documentation](https://docs.rs/routerify-json-response/badge.svg)](https://docs.rs/routerify-json-response)
[![MIT](https://img.shields.io/crates/l/routerify-json-response.svg)](./LICENSE)

# routerify-json-response

A [`Routerify`](https://github.com/routerify/routerify) utility library to generate JSON response.

In `Success` case, It generates JSON response in the following format:
 
```json
{
    "status": "success",
    "code": "<status_code>",
    "data": "<data>"
}
```

In `Failed` case, It generates JSON response in the following format:

```json
{
    "status": "failed",
    "code": "<status_code>",
    "message": "<error_message>"
}
```

[Docs](https://docs.rs/routerify-json-response)

## Install
 
Add this to your `Cargo.toml`:

```toml
[dependencies]
routerify = "3"
routerify-json-response = "3"
```

## Example

```rust
use hyper::{Body, Request, Response, Server, StatusCode};
// Import required routerify_json_response methods.
use routerify_json_response::{json_failed_resp_with_message, json_success_resp};
use routerify::{Router, RouterService};
use std::net::SocketAddr;

async fn list_users_handler(_: Request<Body>) -> Result<Response<Body>, routerify_json_response::Error> {
    // Fetch response data from somewhere.
    let users = ["Alice", "John"];

    // Generate a success JSON response with the data in the following format:
    // { "status": "success", code: 200, data: ["Alice", "John"] }
    json_success_resp(&users)
}

async fn list_books_handler(_: Request<Body>) -> Result<Response<Body>, routerify_json_response::Error> {
    // Generate a failed JSON response in the following format:
    // { "status": "failed", code: 500, data: "Internal Server Error: Couldn't fetch book list from database" }
    json_failed_resp_with_message(
        StatusCode::INTERNAL_SERVER_ERROR,
        "Couldn't fetch book list from database",
    )
}

// Create a router.
fn router() -> Router<Body, routerify_json_response::Error> {
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
    let service = RouterService::new(router).unwrap();

    // The address on which the server will be listening.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    // Create a server by passing the created service to `.serve` method.
    let server = Server::bind(&addr).serve(service);

    println!("App is running on: {}", addr);
    if let Err(err) = server.await {
        eprintln!("Server error: {}", err);
    }
}
```

## Contributing 

Your PRs and suggestions are always welcome.
