use anyhow::Result;
use serde::{Deserialize, Serialize};
use spin_sdk::{
    http::{not_found, Request, Response},
    http_component,
};

#[derive(Debug, Deserialize)]
struct LowercaseRequest {
    value: String,
}

#[derive(Serialize)]
struct LowercaseResponse {
    message: String,
}

/// A simple Spin HTTP component.
#[http_component]
fn lowercase(req: Request) -> Result<Response> {
    let method = req.method();
    if method == "POST" {
        let lowercase_request: LowercaseRequest =
            serde_json::from_slice(req.body().clone().unwrap().to_vec().as_slice()).unwrap();

        let lowercase_response = LowercaseResponse {
            message: lowercase_request.value.to_lowercase(),
        };

        let json_response = serde_json::to_string(&lowercase_response)?;

        return Ok(http::Response::builder()
            .status(200)
            .header("Content-Type", "application/json")
            .body(Some(json_response.into()))?);
    } else {
        return not_found();
    }
}
