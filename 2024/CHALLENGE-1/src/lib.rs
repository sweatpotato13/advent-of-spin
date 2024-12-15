use serde::{Deserialize, Serialize};
use spin_sdk::http::Method;
use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;
use spin_sdk::key_value::Store;

#[derive(Debug, Serialize, Deserialize)]
struct Wishlist {
    name: String,
    items: Vec<String>,
}

#[http_component]
fn handle_request(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("Received request: {:?}", req.method());
    let store = Store::open_default()?;

    let (status, body) = match *req.method() {
        Method::Get => {
            let keys = store.get_keys()?;
            let mut wishlists = vec![];
            for key in keys {
                if let Some(value) = store.get(&key)? {
                    let items: Vec<String> = serde_json::from_slice(&value)?;
                    wishlists.push(Wishlist { name: key, items });
                }
            }
            let json_body = serde_json::to_vec(&wishlists)?;
            (200, json_body)
        }
        Method::Post => {
            let body = req.body();
            let wishlist: Wishlist = match serde_json::from_slice(&body) {
                Ok(wishlist) => wishlist,
                Err(_) => return Ok(Response::builder().status(400).body(Vec::new()).build()),
            };
            let serialized_items = serde_json::to_vec(&wishlist.items)?;
            let _ = store.set(&wishlist.name, &serialized_items);
            (201, Vec::new())
        }
        _ => (404, Vec::new()),
    };

    Ok(Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(body)
        .build())
}
