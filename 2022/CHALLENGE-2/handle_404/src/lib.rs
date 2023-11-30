use anyhow::Result;
use spin_sdk::{
    http::{not_found, Request, Response},
    http_component,
};

/// A simple Spin HTTP component.
#[http_component]
fn handle_404(req: Request) -> Result<Response> {
    return not_found();
}
