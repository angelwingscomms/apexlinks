use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Zone {
    pub l: String,        // location url
    pub n: String,        // name
    pub i: Vec<String>,   // image urls array
    pub p: Position,      // position lat and long
    pub s: String,        // tenant id, constant: "z"
    pub t: String,        // description
    pub embedding: Vec<f32>, // made from zone name and description as json
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub t: String,        // user about/description
    pub l: String,        // user location Google Maps url
    pub i: Vec<String>,   // array of image urls
    pub z: Option<String>, // zone id this user belongs to
    pub u: String,        // username
    pub n: String,        // name
    pub p: Position,      // position lat and long
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub t: String,        // description of product
    pub c: f64,           // price of product
    pub u: String,        // user this product belongs to
    pub z: Option<String>, // zone this product belongs to (inherits from user's zone)
    pub images: Vec<String>, // array of image urls
    pub l: String,        // location url (inherits from user's location)
    pub p: Position,      // position lat and long (inherits from user's position)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
    pub t: String,        // description of service
    pub c: f64,           // price of service
    pub u: String,        // user this service belongs to
    pub z: Option<String>, // zone this service belongs to (inherits from user's zone)
    pub images: Vec<String>, // array of image urls
    pub l: String,        // location url (inherits from user's location)
    pub p: Position,      // position lat and long (inherits from user's position)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Position {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZoneAddRequest {
    pub name: String,
    pub location_url: String,
    pub description: String,
    pub images: Option<Vec<String>>,
    pub position: Position,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZoneSearchRequest {
    pub query: String,
    pub lat: Option<f64>,
    pub lng: Option<f64>,
    pub radius: Option<f64>, // in miles
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserJoinZoneRequest {
    pub zone_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLeaveZoneRequest {
    pub zone_id: String,
} 