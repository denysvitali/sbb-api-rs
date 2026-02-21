use chrono::Utc;
use simple_error::{bail, SimpleError};
use reqwest::Url;

use crate::make_request;
use crate::models::location::SearchDateTimeType;
use crate::models::trip::TripSearchResponse;

/// Fetch connections between two places by name.
///
/// `from_ref` / `to_ref` are optional UIC station IDs (e.g. `"8503000"` for Zürich HB).
/// Providing them yields more reliable results; omit when only the name is known.
pub async fn get_connections(
    from: &str,
    from_ref: Option<&str>,
    to: &str,
    to_ref: Option<&str>,
    on: &chrono::DateTime<Utc>,
    dt_type: SearchDateTimeType,
) -> Result<TripSearchResponse, SimpleError> {
    let date = on.format("%Y-%m-%d").to_string();
    let time = on.format("%H:%M").to_string();

    let mut params: Vec<(&str, &str)> = vec![
        ("departureName", from),
        ("arrivalName", to),
        ("searchDate", &date),
        ("searchTime", &time),
    ];

    // dt_type is a local Display-able value; build the string before borrowing params
    let dt_str = dt_type.to_string();
    params.push(("searchDateTimeType", &dt_str));

    if let Some(r) = from_ref {
        params.push(("departureReference", r));
    }
    if let Some(r) = to_ref {
        params.push(("arrivalReference", r));
    }

    let base_path = format!("/api/timetable/v2/trips");
    let url = Url::parse_with_params(
        &format!("{}{}", crate::API_ENDPOINT, base_path),
        &params,
    )
    .map_err(|e| SimpleError::new(format!("URL parse error: {}", e)))?;

    let resp = make_request(url, &base_path)
        .await
        .map_err(|e| SimpleError::new(format!("Request error: {}", e)))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        bail!("HTTP {}: {}", status, body)
    }

    let text = resp
        .text()
        .await
        .map_err(|e| SimpleError::new(format!("Response read error: {}", e)))?;

    serde_json::from_str(&text)
        .map_err(|e| SimpleError::new(format!("JSON parse error: {}", e)))
}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, TimeZone, Utc};
    use crate::connections::get_connections;
    use crate::models::location::SearchDateTimeType;

    #[actix_rt::test]
    #[ignore = "requires live API access"]
    pub async fn test_get_connection_zh_bs() {
        let today = chrono::offset::Local::now();
        let date = Utc.with_ymd_and_hms(today.year(), today.month(), today.day(), 12, 0, 0).unwrap();
        let result = get_connections(
            "Zürich HB", Some("8503000"),
            "Basel SBB", Some("8500010"),
            &date,
            SearchDateTimeType::Departure,
        )
        .await;
        assert!(result.is_ok(), "error: {:?}", result.err());
        let resp = result.unwrap();
        assert!(!resp.trips.is_empty());
        println!("Got {} trips", resp.trips.len());
        for trip in &resp.trips {
            println!("{:?}", trip.summary.departure_anchor.display_time);
        }
    }

    #[actix_rt::test]
    #[ignore = "requires live API access"]
    pub async fn test_get_connection_by_name_only() {
        let today = chrono::offset::Local::now();
        let date = Utc.with_ymd_and_hms(today.year(), today.month(), today.day(), 12, 0, 0).unwrap();
        let result = get_connections(
            "Zürich HB", None,
            "Bern", None,
            &date,
            SearchDateTimeType::Departure,
        )
        .await;
        assert!(result.is_ok(), "error: {:?}", result.err());
        println!("Trips: {}", result.unwrap().trips.len());
    }
}
