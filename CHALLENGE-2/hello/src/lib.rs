use anyhow::Result;
use serde::{Deserialize, Serialize};
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

#[derive(Serialize)]
struct LowercaseRequest {
    value: String,
}

#[derive(Debug, Deserialize)]
struct LowercaseResponse {
    message: String,
}

#[derive(Serialize)]
struct HelloResponse {
    message: String,
}

/// A simple Spin HTTP component.
#[http_component]
fn hello(req: Request) -> Result<Response> {
    let default_value: &str = "world";

    let name: &str = req
        .headers()
        .get("spin-path-info")
        .unwrap()
        .to_str()
        .unwrap();

    let mut hello_response = HelloResponse {
        message: format!("Hello, {}!", default_value),
    };

    if name != "" {
        let lowercase_request = LowercaseRequest {
            value: name.strip_prefix("/").unwrap().to_string(),
        };
        let host: &str = req.headers().get("host").unwrap().to_str().unwrap();
        let spin_full_url: &str = req
            .headers()
            .get("spin-full-url")
            .unwrap()
            .to_str()
            .unwrap();

        let protocol = spin_full_url.split_once("://").unwrap().0;

        let uri = format!("{}://{}/lowercase", protocol, host);
        println!("{:?}", uri);
        let res = spin_sdk::outbound_http::send_request(
            http::Request::builder().method("POST").uri(uri).body(Some(
                serde_json::to_string(&lowercase_request).unwrap().into(),
            ))?,
        )?;

        let lowercase_response: LowercaseResponse =
            serde_json::from_slice(res.body().clone().unwrap().to_vec().as_slice()).unwrap();

        hello_response.message = format!("Hello, {}!", lowercase_response.message);
    }

    let json_response = serde_json::to_string(&hello_response)?;

    Ok(http::Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(Some(json_response.into()))?)
}
