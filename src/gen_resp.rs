use hyper::{body::HttpBody, header, Response, StatusCode};
use serde::Serialize;

pub(crate) fn gen_response<B, D>(code: StatusCode, resp_data: &D) -> routerify::Result<Response<B>>
where
    B: HttpBody + From<Vec<u8>> + Send + Sync + Unpin + 'static,
    D: Serialize + Send + Sync + Unpin,
{
    let json_resp_data = match serde_json::to_vec(&resp_data) {
        Ok(json_data) => json_data,
        Err(err) => {
            return Err(routerify::Error::new(format!(
                "routerify-json-response: Failed to convert the response data as JSON: {}",
                err
            )));
        }
    };

    let content_ln = json_resp_data.len();
    let body = B::from(json_resp_data);

    let resp = Response::builder()
        .status(code)
        .header(header::CONTENT_LENGTH, content_ln.to_string())
        .header(header::CONTENT_TYPE, "application/json; charset=utf-8")
        .body(body);

    match resp {
        Ok(resp) => Ok(resp),
        Err(err) => Err(routerify::Error::new(format!(
            "routerify-json-response: Failed to create response: {}",
            err
        ))),
    }
}
