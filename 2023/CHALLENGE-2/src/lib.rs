use http::{Method, StatusCode};
use serde::{Deserialize, Serialize};
use spin_sdk::{
    http::{IntoResponse, Response},
    http_component,
};

#[derive(Debug, Deserialize, Serialize)]
struct KidsRequest {
    kids: Vec<usize>,
    weight: Vec<usize>,
    capacity: usize,
}

#[derive(Debug, Deserialize, Serialize)]
struct KidsResponse {
    kids: usize,
}

fn optimal_kids_reached(kids: Vec<usize>, weight: Vec<usize>, capacity: usize) -> usize {
    let n = kids.len();
    let mut dp = vec![vec![0; capacity + 1]; n + 1];

    for i in 1..=n {
        for w in 1..=capacity {
            if weight[i - 1] <= w {
                dp[i][w] = dp[i - 1][w].max(kids[i - 1] + dp[i - 1][w - weight[i - 1]]);
            } else {
                dp[i][w] = dp[i - 1][w];
            }
        }
    }

    dp[n][capacity]
}

#[http_component]
fn handle_request(req: http::Request<Vec<u8>>) -> anyhow::Result<impl IntoResponse> {
    let (status, body) = match *req.method() {
        Method::POST => {
            let request: KidsRequest =
                serde_json::from_slice(req.body().clone().to_vec().as_slice()).unwrap();

            let kids = request.kids;
            let weight = request.weight;
            let capacity = request.capacity;

            let maximum_kids = optimal_kids_reached(kids, weight, capacity);

            let mut response = KidsResponse { kids: maximum_kids };

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
