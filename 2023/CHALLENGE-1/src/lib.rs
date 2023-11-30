use http::{Method, StatusCode};
use spin_sdk::{
    http::{IntoResponse, Response},
    http_component,
    key_value::Store,
};

#[http_component]
fn handle_request(req: http::Request<Vec<u8>>) -> anyhow::Result<impl IntoResponse> {
    let store = Store::open_default()?;

    let (status, body) = match *req.method() {
        Method::POST => {
            store.set(req.uri().path(), req.body().as_slice())?;
            println!(
                "Storing value in the KV store with {:?} as the key",
                req.uri().path()
            );
            (StatusCode::CREATED, None)
        }
        Method::GET => match store.get(req.uri().path())? {
            Some(value) => {
                println!("Found value for the key {:?}", req.uri().path());
                (StatusCode::OK, Some(value))
            }
            None => {
                println!("No value found for the key {:?}", req.uri().path());
                (StatusCode::NOT_FOUND, None)
            }
        },
        Method::DELETE => {
            store.delete(req.uri().path())?;
            println!("Delete key {:?}", req.uri().path());
            (StatusCode::OK, None)
        }
        Method::HEAD => {
            let code = if store.exists(req.uri().path())? {
                println!("{:?} key found", req.uri().path());
                StatusCode::OK
            } else {
                println!("{:?} key not found", req.uri().path());
                StatusCode::NOT_FOUND
            };
            (code, None)
        }
        _ => (StatusCode::METHOD_NOT_ALLOWED, None),
    };
    Ok(Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(body)
        .build())
}
