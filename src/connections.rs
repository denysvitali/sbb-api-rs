use chrono::{Utc, TimeZone, Datelike};
use crate::models::results::VerbindungenResults;
use crate::make_request;
use crate::models::location::LocationType;
use simple_error::SimpleError;

pub fn get_connections(from: &str, from_type: LocationType, to: &str, to_type: LocationType,
                       on: &chrono::DateTime<Utc>) -> Result<VerbindungenResults, SimpleError> {
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

    if !resp.status().is_success() {
       bail!("status is not success")
    }

    let response = &resp.text().unwrap();

    Ok(serde_json::from_str(
        response
    ).unwrap_or(VerbindungenResults {
        verbindungen: vec![],
        earlier_url: None,
        late_url: None,
        verbindung_preis_url: "".to_string(),
    }))
}


#[test]
pub fn test_get_connection() {
    let today = chrono::offset::Local::now();
    let date = Utc.ymd(today.year(),  today.month(), today.day()).and_hms(12, 0, 0);
    let conn = get_connections("Zürich HB",
                               LocationType::Station,
                               "Basel",
                               LocationType::Station,
                               &date);
    assert!(conn.is_ok());
    let verbindungen_res = conn.unwrap();
    assert!(verbindungen_res.verbindungen.len() > 0);
    println!("Connections = {:?}", verbindungen_res);
}

#[test]
pub fn test_get_connection2() {
    let today = chrono::offset::Local::now();
    let date = Utc.ymd(today.year(),  today.month(), today.day()).and_hms(12, 0, 0);
    let conn = get_connections("Chiasso",
                               LocationType::Station,
                               "Zürich HB",
                               LocationType::Station,
                               &date);
    assert!(conn.is_ok());

    let verbindungen_res = conn.unwrap();
    assert!(verbindungen_res.verbindungen.len() > 0);
    println!("Connections = {:?}", verbindungen_res);
}

#[test]
pub fn test_get_connections3() {
    let today = chrono::offset::Local::now();
    let date = Utc.ymd(today.year(),  today.month(), today.day()).and_hms(12, 0, 0);
    let conn = get_connections("Zürich HB",
                               LocationType::Station,
                               "Dübendorf, Bahnof",
                               LocationType::Station,
                               &date,
    );

    assert!(conn.is_ok());
    println!("Connections = {}", conn.unwrap());
}

#[test]
pub fn test_get_connections4() {
    let today = chrono::offset::Local::now();
    let date = Utc.ymd(today.year(),  today.month(), today.day()).and_hms(12, 0, 0);
    let conn = get_connections("Chiasso",
                               LocationType::Address,
                               "Zürich",
                               LocationType::Address,
                               &date,
    );

    assert!(conn.is_ok());

    let verbindungen_res = conn.unwrap();
    println!("Connections = {}", verbindungen_res);

    for c in verbindungen_res.verbindungen {
        println!("Conn: {}, Duration = {:?}", c, c.duration());
    }
}
