use spin_sdk::http::{IntoResponse, Method, Request, Response};
use spin_sdk::http_component;

#[http_component]
fn handle_request(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("Received request: {:?}", req.method());

    let (status, body) = match *req.method() {
        Method::Get => {
            let json_body = serde_json::json!({
                "favorite": {
                    "music": "https://music.apple.com/de/playlist/advent-of-spin-2024/pl.u-xky2TDpLaZ?l=en-GB"
                }
            });
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
