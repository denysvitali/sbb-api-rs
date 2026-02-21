use serde::{Deserialize, Serialize};

/// Response from `GET /api/timetable/v2/places`.
/// The API returns a JSON array of places.
pub type PlaceSearchResponse = Vec<Place>;

/// A location returned by the places search endpoint.
/// Corresponds to `PlaceDto` in the Android app.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Place {
    pub display_name: String,
    /// UIC station reference (e.g. "8503000" for Zürich HB). May be absent for addresses/POIs.
    pub identifier: Option<String>,
    /// Place type: "STOP_PLACE", "ADDRESS", "POI", etc.
    pub place_type: String,
    pub coordinates: Coordinates,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}
