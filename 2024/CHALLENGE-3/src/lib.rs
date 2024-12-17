use serde::{Deserialize, Serialize};
use spin_sdk::http::Method;
use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

mod bindings;

#[derive(Debug, Serialize, Deserialize)]
struct SuggestionsRequest {
    name: String,
    age: u8,
    likes: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SuggestionsResonse {
    name: String,
    #[warn(non_snake_case)]
    giftSuggestions: String,
}

#[http_component]
fn handle_request(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("Received request: {:?}", req.method());

    let (status, body) = match *req.method() {
        Method::Post => {
            let body = req.body();
            let suggestions_request: SuggestionsRequest = match serde_json::from_slice(&body) {
                Ok(suggestions_request) => suggestions_request,
                Err(_) => return Ok(Response::builder().status(400).body(Vec::new()).build()),
            };
            let gift_suggestions = bindings::deps::components::advent_of_spin::generator::suggest(
                &suggestions_request.name,
                suggestions_request.age,
                &suggestions_request.likes,
            )
            .unwrap();
            let json_body = SuggestionsResonse {
                name: gift_suggestions.clone().name,
                giftSuggestions: gift_suggestions.clone().suggestions,
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
