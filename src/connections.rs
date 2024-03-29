use chrono::{Utc};
use simple_error::SimpleError;

use crate::make_request;
use crate::models::location::LocationType;
use crate::models::results::VerbindungenResults;

pub async fn get_connections(from: &str, from_type: LocationType, to: &str, to_type: LocationType,
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

    let resp = make_request(&path).await.expect("Invalid request");

    if !resp.status().is_success() {
        bail!("status is not success")
    }

    let response = &resp.text().await.unwrap();

    Ok(serde_json::from_str(
        response
    ).unwrap())
}

mod tests {
    use chrono::{Datelike, TimeZone, Utc};
    use crate::connections::get_connections;
    use crate::models::location::LocationType;

    #[actix_rt::test]
    pub async fn test_get_connection() {
        let today = chrono::offset::Local::now();
        let date = chrono::Utc.ymd(today.year(), today.month(), today.day()).and_hms(12, 0, 0);
        let conn = get_connections("Zürich HB",
                                   LocationType::Station,
                                   "Basel",
                                   LocationType::Station,
                                   &date);
        let conn_res = conn.await;
        assert!(conn_res.is_ok());
        let verbindungen_res = conn_res.unwrap();
        assert!(verbindungen_res.verbindungen.len() > 0);
        println!("Connections = {:?}", verbindungen_res);
    }

    #[actix_rt::test]
    pub async fn test_get_connection2() {
        let today = chrono::offset::Local::now();
        let date = Utc.ymd(today.year(), today.month(), today.day()).and_hms(12, 0, 0);
        let conn = get_connections("Chiasso",
                                   LocationType::Station,
                                   "Zürich HB",
                                   LocationType::Station,
                                   &date);
        let conn_res = conn.await;
        assert!(conn_res.is_ok());
        let verbindungen_res = conn_res.unwrap();
        assert!(verbindungen_res.verbindungen.len() > 0);
        println!("Connections = {:?}", verbindungen_res);
    }

    #[actix_rt::test]
    pub async fn test_get_connections3() {
        let today = chrono::offset::Local::now();
        let date =
            Utc.ymd(today.year(), today.month(), today.day())
                .and_hms(12, 0, 0);
        let conn = get_connections(
            "Zürich HB",
            LocationType::Station,
            "Dübendorf, Bahnof",
            LocationType::Station,
            &date,
        );

        let conn_res = conn.await;
        assert!(conn_res.is_ok());
        let verbindungen_res = conn_res.unwrap();
        println!("Connections = {}", verbindungen_res)
    }

    #[actix_rt::test]
    pub async fn test_get_connections4() {
        let today = chrono::offset::Local::now();
        let date =
            Utc.ymd(today.year(), today.month(), today.day())
                .and_hms(12, 0, 0);
        let conn = get_connections("Chiasso",
                                   LocationType::Address,
                                   "Zürich",
                                   LocationType::Address,
                                   &date,
        );

        let conn_res = conn.await;
        assert!(conn_res.is_ok());
        let verbindungen_res = conn_res.unwrap();
        println!("Connections = {}", verbindungen_res);

        for c in verbindungen_res.verbindungen {
            println!("Conn: {}, Duration = {:?}", c, c.duration());
        }
    }

    #[actix_rt::test]
    pub async fn test_get_connections5() {
        let today = chrono::offset::Local::now();
        let date =
            Utc.ymd(today.year(), today.month(), today.day())
                .and_hms(12, 0, 0);
        let conn = get_connections("Im Tiergraten, 8055 Zürich, Zürich",
                                   LocationType::Address,
                                   "8005 Zürich, Hardturmstrasse 3",
                                   LocationType::Address,
                                   &date,
        );

        let conn_res = conn.await;
        assert!(conn_res.is_ok());
        let verbindungen_res = conn_res.unwrap();
        println!("Connections = {}", verbindungen_res);

        for c in verbindungen_res.verbindungen {
            println!("Conn: {}, Duration = {:?}", c, c.duration());
        }
    }
}