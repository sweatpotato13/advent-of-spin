use anyhow::Result;
use http::Method;
use serde::{Deserialize, Serialize};
use spin_sdk::{
    http::{not_found, Request, Response},
    http_component,
    pg::{self, Decode},
};

const DB_URL_ENV: &str = "DB_URL";

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Data {
    value: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct DataInt {
    value: i32,
}

/// A simple Spin HTTP component.
#[http_component]
fn challenge(req: Request) -> Result<Response> {
    match req.method().to_owned() {
        Method::GET => read(req),
        Method::PUT => write(req),
        _ => not_found(),
    }
}

fn read(_req: Request) -> Result<Response> {
    let key: &str = _req
        .headers()
        .get("spin-path-info")
        .unwrap()
        .to_str()
        .unwrap()
        .strip_prefix('/')
        .unwrap();

    let mut value_response = DataInt { value: 0 };

    let address = std::env::var(DB_URL_ENV)?;

    let sql = format!("SELECT value FROM dev WHERE key='{}'", key);
    let rowset = pg::query(&address, &sql[..], &[])?;
    for row in rowset.rows {
        let value = i32::decode(&row[0])?;
        value_response.value = value;
    }

    let json_response = serde_json::to_string(&value_response)?;

    return Ok(http::Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(Some(json_response.into()))?);
}

fn write(_req: Request) -> Result<Response> {
    let key: &str = _req
        .headers()
        .get("spin-path-info")
        .unwrap()
        .to_str()
        .unwrap()
        .strip_prefix('/')
        .unwrap();

    let value_request: Data =
        serde_json::from_slice(_req.body().clone().unwrap().to_vec().as_slice()).unwrap();

    let address = std::env::var(DB_URL_ENV)?;

    let sql = format!(
        "INSERT INTO dev (key, value) VALUES ('{}', '{}')",
        key, value_request.value
    );
    let executed = pg::execute(&address, &sql[..], &[])?;

    return Ok(http::Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(Some("done".into()))?);
}
