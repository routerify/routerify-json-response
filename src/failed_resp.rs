use crate::gen_resp::gen_response;
use hyper::{body::HttpBody, Response, StatusCode};
use serde::Serialize;

const STATUS_FAILED: &'static str = "failed";

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct FailedResp {
    status: &'static str,
    code: u16,
    message: String,
}

/// Generates a failed JSON response with the provided message and status code.
///
/// It generates JSON response in the following JSON format:
///
///  ```json
///  {
///      "status": "failed",
///      "code": "<status_code>",
///      "message": "<error_message>"
///  }
///```
///
/// # Examples
///
/// ```
/// use hyper::{Body, Request, Response, StatusCode};
/// use json_response::{json_failed_resp_with_message};
///
/// async fn list_books_handler(_: Request<Body>) -> Result<Response<Body>, routerify::Error> {
///     // Generate a failed JSON response in the following format:
///     // { "status": "failed", code: 500, data: "Internal Server Error: Couldn't fetch book list from database" }
///     json_failed_resp_with_message(
///         StatusCode::INTERNAL_SERVER_ERROR,
///         "Couldn't fetch book list from database",
///      )
/// }
/// ```
pub fn json_failed_resp_with_message<B, M>(code: StatusCode, message: M) -> routerify::Result<Response<B>>
where
    B: HttpBody + From<Vec<u8>> + Send + Sync + Unpin + 'static,
    M: Into<String>,
{
    let resp_data = FailedResp {
        status: STATUS_FAILED,
        code: code.as_u16(),
        message: format!("{}: {}", code.canonical_reason().unwrap(), message.into()),
    };

    gen_response(code, &resp_data)
}

/// Generates a failed JSON response with the status code specific message and status code.
///
/// It generates JSON response in the following JSON format:
///
///  ```json
///  {
///      "status": "failed",
///      "code": "<status_code>",
///      "message": "<status_code_message>"
///  }
///```
///
/// # Examples
///
/// ```
/// use hyper::{Body, Request, Response, StatusCode};
/// use json_response::{json_failed_resp};
///
/// async fn list_books_handler(_: Request<Body>) -> Result<Response<Body>, routerify::Error> {
///     // Generate a failed JSON response in the following format:
///     // { "status": "failed", code: 500, data: "Internal Server Error" }
///     json_failed_resp(StatusCode::INTERNAL_SERVER_ERROR)
/// }
/// ```
pub fn json_failed_resp<B>(code: StatusCode) -> routerify::Result<Response<B>>
where
    B: HttpBody + From<Vec<u8>> + Send + Sync + Unpin + 'static,
{
    let resp_data = FailedResp {
        status: STATUS_FAILED,
        code: code.as_u16(),
        message: code.canonical_reason().unwrap().to_string(),
    };

    gen_response(code, &resp_data)
}
