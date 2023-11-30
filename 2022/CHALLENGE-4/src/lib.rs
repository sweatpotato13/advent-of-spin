extern crate haversine;
use anyhow::Result;
use http::*;
use serde::{Deserialize, Serialize};
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Coordinate {
    lat: f64,
    long: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DistanceRequest {
    d1: Coordinate,
    d2: Coordinate,
}

#[derive(Debug, Serialize)]
pub struct DistanceResponse {
    distance: f64,
}

/// A simple Spin HTTP component.
#[http_component]
fn distance_latlog(req: Request) -> Result<Response> {
    let distance_request: DistanceRequest =
        serde_json::from_slice(req.body().clone().unwrap().to_vec().as_slice()).unwrap();

    let d1: Coordinate = distance_request.d1;
    let d2: Coordinate = distance_request.d2;
    let start1 = haversine::Location {
        latitude: d1.lat,
        longitude: d1.long,
    };
    let end1 = haversine::Location {
        latitude: d2.lat,
        longitude: d2.long,
    };
    let distance = haversine::distance(start1, end1, haversine::Units::Miles);

    let nautical_miles = unit_conversions::length::miles::to_nautical_miles(distance);

    let rounded_miles = (nautical_miles * 10.0).round() / 10.0;

    let distance_response = DistanceResponse {
        distance: rounded_miles,
    };

    let json_response = serde_json::to_string(&distance_response)?;

    Ok(http::Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(Some(json_response.into()))?)
}
