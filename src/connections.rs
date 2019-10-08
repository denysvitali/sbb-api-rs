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

    let mut resp = make_request(&path).expect("Invalid request");
    let response = &resp.text().unwrap();

    serde_json::from_str(
        response
    ).unwrap_or(VerbindungenResults{
        verbindungen: vec![],
        earlier_url: None,
        late_url: None,
        verbindung_preis_url: "".to_string()
    })
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

#[test]
pub fn test_get_connection2() {
    let date = Utc.ymd(2019, 9, 28).and_hms(12, 0, 0);
    let conn = get_connections("Chiasso",
                               LocationType::Station,
                               "Zürich HB",
                               LocationType::Station,
                               &date);
    assert!(conn.verbindungen.len() > 0);
    println!("Connections = {:?}", conn);
}

#[test]
pub fn test_get_connections3() {
    let date = Utc.ymd(2019, 10, 8).and_hms(21,31,0);
    let conn = get_connections("Zürich HB",
            LocationType::Station,
            "Dübendorf, Bahnof",
            LocationType::Station,
            &date
    );

    println!("Connections = {}", conn);
}

#[test]
pub fn test_get_connections4() {
    let date = Utc.ymd(2019, 10, 9).and_hms(8, 0, 0);
    let conn = get_connections("Chiasso",
            LocationType::Address,
            "Zürich",
            LocationType::Address,
            &date
    );

    println!("Connections = {}", conn);

    for c in conn.verbindungen {
        println!("Conn: {}, Duration = {:?}", c, c.duration());
    }
}