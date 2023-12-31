use http::{Method, StatusCode};
use serde::{Deserialize, Serialize};
use spin_sdk::{
    http::{IntoResponse, Response},
    http_component, llm,
};

#[derive(Debug, Deserialize, Serialize)]
struct StoryRequest {
    place: String,
    characters: Vec<String>,
    objects: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct StoryResponse {
    story: String,
}

fn make_story(place: String, characters: Vec<String>, objects: Vec<String>) -> String {
    let model = llm::InferencingModel::Llama2Chat;
    let prompt = format!(
        "Can you make story with place {:?} and these characters {:?} and this objects {:?}",
        place, characters, objects
    );
    let inference = llm::infer(model, &prompt);
    format!("{:?}", inference)
}

#[http_component]
fn handle_request(req: http::Request<Vec<u8>>) -> anyhow::Result<impl IntoResponse> {
    let (status, body) = match *req.method() {
        Method::POST => {
            let request: StoryRequest =
                serde_json::from_slice(req.body().clone().to_vec().as_slice()).unwrap();

            let place = request.place;
            let characters = request.characters;
            let objects = request.objects;

            let story = make_story(place, characters, objects);

            let response = StoryResponse { story: story };

            let json_response = serde_json::to_string(&response)?;

            (StatusCode::OK, json_response)
        }
        _ => (StatusCode::METHOD_NOT_ALLOWED, String::new()),
    };
    Ok(Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(body)
        .build())
}
