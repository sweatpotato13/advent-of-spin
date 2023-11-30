use anyhow::Result;
use serde::{Deserialize, Serialize};
use spin_sdk::{
    http::{internal_server_error, Request, Response},
    http_component,
};

const DISCORD_WEBHOOK_ENV: &str = "DISCORD_WEBHOOK";

#[derive(Debug, Deserialize)]
struct WebhookRequest {
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DiscordRequest {
    content: String,
}

/// A simple Spin HTTP component.
#[http_component]
fn webhook(req: Request) -> Result<Response> {
    if let Err(e) = handle_webhook(req) {
        eprintln!("Error handling webhook: {}", e);
        return internal_server_error();
    }

    Ok(http::Response::builder()
        .status(200)
        .body(Some("OK".into()))?)
}

fn handle_webhook(req: Request) -> Result<()> {
    let webhook_endpoint = std::env::var(DISCORD_WEBHOOK_ENV);

    let body = req.body().clone().unwrap_or_default();
    let webhook: WebhookRequest = serde_json::from_slice(&body)?;

    let data = webhook.value;

    let msg = format!("Hello {}", data);

    let discord_request = DiscordRequest { content: msg };

    let res = spin_sdk::outbound_http::send_request(
        http::Request::builder()
            .method("POST")
            .header("Content-Type", "application/json")
            .uri(webhook_endpoint.clone().unwrap())
            .body(Some(
                serde_json::to_string(&discord_request).unwrap().into(),
            ))?,
    )?;

    Ok(())
}
