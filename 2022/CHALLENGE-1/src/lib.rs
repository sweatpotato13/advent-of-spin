use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

/// A simple Spin HTTP component.
#[http_component]
fn challenge(req: Request) -> Result<Response> {
    println!("{:?}", req.headers());
    let data = r#"{
        "message": "Hello, world!"
    }"#;
    Ok(http::Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(Some(data.into()))?)
}
