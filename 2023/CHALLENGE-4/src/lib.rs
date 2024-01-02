use http::StatusCode;
use serde::{Deserialize, Serialize};
use spin_sdk::{
    http::{IntoResponse, Method, Request, Response},
    http_component,
};

#[derive(Debug, Deserialize, Serialize)]
struct GameResponse {
    cows: u64,
    bulls: u64,
    gameId: String,
    guesses: u64,
    solved: bool,
}

#[http_component]
async fn handle_request(_req: http::Request<Vec<u8>>) -> anyhow::Result<impl IntoResponse> {
    let guess_numbers = [
        "013", "014", "021", "023", "024", "031", "032", "034", "102", "103", "104", "120",
        "123", "124", "130", "132", "134", "201", "203", "204", "210", "213", "214", "230", "231",
        "234", "301", "302", "304", "310", "312", "314", "320", "321", "324",
    ];

    let status = StatusCode::OK;
    let body = "Done".to_string();

    let uri = format!("https://bulls-n-cows.fermyon.app/api?guess=012");

    let req = Request::builder().method(Method::Get).uri(uri).build();

    // Send the request and await the response
    let res: Response = spin_sdk::http::send(req).await?;

    let response: GameResponse = serde_json::from_slice(res.body().to_vec().as_slice()).unwrap();     

    if response.solved {
        println!("{:?}", response);
        Ok(Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(body)
        .build())
    }
    else {
        let game_id = response.gameId;
        for guess in guess_numbers.iter() {
            let uri = format!("https://bulls-n-cows.fermyon.app/api?guess={}&id={}", guess, game_id);
        
            let req = Request::builder().method(Method::Get).uri(uri).build();
        
            // Send the request and await the response
            let res: Response = spin_sdk::http::send(req).await?;
        
            let response: GameResponse = serde_json::from_slice(res.body().to_vec().as_slice()).unwrap();     
    
            if response.solved {
                println!("{:?}", response);
                break;
            }
        }
        Ok(Response::builder()
            .status(status)
            .header("content-type", "application/json")
            .body(body)
            .build())
    
    }
}
