use serde::{Deserialize, Serialize};
use spin_sdk::http::Method;
use spin_sdk::http::StatusCode;
use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

#[derive(Debug, Serialize, Deserialize)]
struct NaughtyOrNice {
    name: String,
    score: u8,
}

#[http_component]
fn handle_request(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("Received request: {:?}", req.method());

    let (status, body) = match *req.method() {
        Method::Get => {
            println!("GET request received");
            let path = req.uri();
            let parts: Vec<&str> = path.split('/').collect();
            let name = parts[5];
            println!("param: {:?}", name);
            let json_body = NaughtyOrNice {
                name: name.
                score: 99,
            };
            (200, serde_json::to_vec(&json_body).unwrap())
        }
        _ => (404, Vec::new()),
    };

    Ok(Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(body)
        .build())
}
