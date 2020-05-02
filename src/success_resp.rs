use crate::gen_resp::gen_response;
use hyper::{body::HttpBody, Response, StatusCode};
use serde::Serialize;

const STATUS_SUCCESS: &'static str = "success";

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct SuccessResp<'a, D>
where
    D: Serialize + Send + Sync + Unpin,
{
    status: &'static str,
    code: u16,
    data: &'a D,
}

/// Generates a success JSON response with the provided data and the status code.
///
/// It generates JSON response in the following JSON format:
///
///  ```json
///  {
///      "status": "success",
///      "code": "<status_code>",
///      "data": "<data>"
///  }
///```
///
/// # Examples
///
/// ```
/// use hyper::{Body, Request, Response, StatusCode};
/// use json_response::{json_success_resp_with_code};
///
/// async fn list_users_handler(_: Request<Body>) -> Result<Response<Body>, routerify::Error> {
///     // Fetch response data from somewhere.
///     let users = ["Alice", "John"];
///
///     // Generate a success JSON response with the data in the following format:
///     // { "status": "success", code: 201, data: ["Alice", "John"] }
///     json_success_resp_with_code(StatusCode::CREATED, &users)
/// }
/// ```
pub fn json_success_resp_with_code<B, D>(code: StatusCode, data: &D) -> routerify::Result<Response<B>>
where
    B: HttpBody + From<Vec<u8>> + Send + Sync + Unpin + 'static,
    D: Serialize + Send + Sync + Unpin,
{
    let resp_data = SuccessResp {
        status: STATUS_SUCCESS,
        code: code.as_u16(),
        data,
    };

    gen_response(code, &resp_data)
}

/// Generates a success JSON response with the provided data and the `OK 200` status code.
///
/// It generates JSON response in the following JSON format:
///
///  ```json
///  {
///      "status": "success",
///      "code": "200",
///      "data": "<data>"
///  }
///```
///
/// # Examples
///
/// ```
/// use hyper::{Body, Request, Response, StatusCode};
/// use json_response::{json_success_resp};
///
/// async fn list_users_handler(_: Request<Body>) -> Result<Response<Body>, routerify::Error> {
///     // Fetch response data from somewhere.
///     let users = ["Alice", "John"];
///
///     // Generate a success JSON response with the data in the following format:
///     // { "status": "success", code: 200, data: ["Alice", "John"] }
///     json_success_resp(&users)
/// }
/// ```
pub fn json_success_resp<B, D>(data: &D) -> routerify::Result<Response<B>>
where
    B: HttpBody + From<Vec<u8>> + Send + Sync + Unpin + 'static,
    D: Serialize + Send + Sync + Unpin,
{
    json_success_resp_with_code::<B, D>(StatusCode::OK, data)
}
