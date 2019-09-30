use chrono::{Utc, TimeZone};
use crate::models::results::VerbindungenResults;
use crate::make_request;
use crate::models::location::LocationType;
use percent_encoding::{utf8_percent_encode, CONTROLS, AsciiSet};
use std::process::exit;

pub fn get_connections(from: &str, from_type: LocationType, to: &str, to_type: LocationType,
                       on: &chrono::DateTime<Utc>) -> VerbindungenResults {
    let base_path = "/unauth/fahrplanservice/v1/verbindungen/";

    // s/Zurich/s/Bern/ab/2019-09-20/10-14/";
    let date = on.format("%Y-%m-%d");
    let time = on.format("%H-%M");

    let addition = format!("{}/{}/{}/{}/ab/{}/{}/", from_type,
                           from,
                           to_type,
                           to,
                           date,
                           time);
    let path = format!("{}{}", base_path, addition);

    println!("Path is: {}", path);
    let mut resp = make_request(&path).expect("Invalid request");
    let response = &resp.text().unwrap();

    println!("Response is {}", response);

    serde_json::from_str(
        response
    ).unwrap()
}


#[test]
pub fn test_get_connection() {
    let date = Utc.ymd(2019, 9, 28).and_hms(12, 0, 0);
    let conn = get_connections("Zürich HB",
                               LocationType::Station,
                               "Basel",
                               LocationType::Station,
                               &date);
    assert!(conn.verbindungen.len() > 0);
    println!("Connections = {:?}", conn);
}