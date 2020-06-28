extern crate openssl;
extern crate hex;
extern crate uuid;
extern crate chrono;
extern crate percent_encoding;
extern crate onig;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate simple_error;


#[cfg(test)]
#[macro_use]
extern crate more_asserts;

pub mod authenticator;
pub mod connections;
pub mod models;


use std::str::FromStr;
use reqwest::{Certificate, Method, Url, Response};
use reqwest::header::{HeaderMap, USER_AGENT, HeaderValue, HeaderName};
use openssl::rand::rand_bytes;
use chrono::{DateTime, Local};

pub const API_ENDPOINT : &str = "https://p1.sbbmobile.ch";
//const API_ENDPOINT: &str = "http://127.0.0.1:3000";
pub const SBB_UA: &str = "SBBmobile/flavorProdRelease-10.8.1 Android/10 (OnePlus;ONEPLUS A5010)";

pub fn set_headers(headers: &mut HeaderMap, path: &str, date: &str){
    headers.append(HeaderName::from_str("X-App-Token")
                       .expect("Unable to parse HeaderName"),
                   HeaderValue::from_str(&generate_token())
                       .expect("Unable to parse HeaderValue"),
    );
    headers.append(HeaderName::from_str("X-API-AUTHORIZATION")
                       .expect("Unable to parse HeaderName"),
                   HeaderValue::from_str(
                       &authenticator::get_authorization(&path, &date)
                   )
                       .expect("Unable to parse HeaderValue"),
    );
    headers.append(HeaderName::from_str("X-API-DATE")
                       .expect("Unable to parse HeaderName"),
                   HeaderValue::from_str(
                       &date
                   )
                       .expect("Unable to parse HeaderValue"),
    );
}

fn make_request(path: &str) -> Result<Response, reqwest::Error> {
    let mut headers = HeaderMap::new();
    headers.append(USER_AGENT, HeaderValue::from_str(SBB_UA)
        .expect("Unable to parse User-Agent as HeaderValue"));
    let client = reqwest::Client::builder()
        .add_root_certificate(
            Certificate::from_der(
                include_bytes!("../resources/ca_cert.crt"))
                .expect("Unable to decode certificate")
        )
        .default_headers(headers)
        .build().expect("Unable to create client");

    let local: DateTime<Local> = Local::now();
    let date = format!("{}", local.format("%Y-%m-%d"));

    let url = Url::parse(&format!("{}{}", API_ENDPOINT, path))
                    .expect("Unable to parse URL");
    let mut request = reqwest::Request::new(Method::GET, url);
    let headers: &mut HeaderMap = request.headers_mut();
    set_headers(headers, &path, &date);
    client.execute(request)
}

pub fn generate_token() -> String {
    let mut bytes = [0; 16];
    let _ = rand_bytes(&mut bytes);
    uuid::builder::Builder::from_slice(&bytes)
        .expect("Unable to get UUID from bytes")
        .build()
        .to_hyphenated().to_string()
}

#[cfg(test)]
mod tests {
    use super::authenticator;
    use crate::make_request;

    #[test]
    fn test_path_1() {
        let path = "/unauth/ticketingservice/zvs/v0/features";
        assert_eq!(authenticator::get_certificate_hash(), "WdfnzdQugRFUF5b812hZl3lAahM=");
        assert_eq!(authenticator::get_authorization(path, "2019-09-05"), "wqhPBCfC9oc8gp62FVVIiNIADzg=");
    }

    /*#[test]
    fn test_path_a() {
        let path = "/unauth/fahrplanservice/v1/standorte/ba/";
        assert_eq!(authenticator::get_authorization(path, "2019-09-23"), "qGkCalmIy1kRb4iJVBDQ/bhAnOQ=");
    }



    #[test]
    fn test_path_b() {
        let path = "/unauth/fahrplanservice/v1/verbindungen/a/8001 Zürich, Stampfenbachstrasse 1/s/Basel/ab/2019-09-23/22-42/";
        assert_eq!(authenticator::get_authorization(path, "2019-09-23"), "isgBYXvgx3mlcBiqPWmTAk/2G3o=" );
    }

    #[test]
    fn test_path_c() {
        let path = "/unauth/fahrplanservice/v1/standorte/Basel/";
        assert_eq!(authenticator::get_authorization(path, "2019-09-23"), "ZwdcDbK99hYBpZ3vhKB9gLOeazw=" );
    }

    #[test]
    fn test_path_2() {
        let path = "/unauth/ticketingservice/zvs/v0/ghettobox/";
        assert_eq!(authenticator::get_authorization(path, "2019-09-05"), "3fgUyXQoMieevNYULWbo3OPsd4w=");
    }

    #[test]
    fn test_path_3() {
        let path = "/unauth/fahrplanservice/v1/standorte/a%20b/";
        assert_eq!(authenticator::get_authorization(path, "2019-09-23"), "myZVEyS8WprlVrTIw2ZynzS0Z6I=");
    }

    #[test]
    fn test_request_1() {
        let path = "/unauth/ticketingservice/zvs/v0/features";
        let mut resp = make_request(path).expect("Invalid request");

        println!("{}", resp.text().expect("Unable to extract text"))
    }

    #[test]
    fn test_request_2() {
        let path = "/unauth/fahrplanservice/v1/standorte/lug/";
        let mut resp = make_request(path).expect("Invalid request");

        println!("{}", resp.text().expect("Unable to extract text"))
    }

    #[test]
    fn test_request_3() {
        let path = "/unauth/fahrplanservice/v1/standortenearby/47378177/8540193/";
        let mut resp = make_request(path).expect("Invalid request");

        println!("{}", resp.text().expect("Unable to extract text"))
    }

    #[test]
    fn test_request_4() {
        // yyyy-MM-dd HH-mm
        let path = "/unauth/v1/shortlink/verbindung/008503000/2019-09-20 08:00/";
        let mut resp = make_request(path).expect("Invalid request");

        println!("{}", resp.text().expect("Unable to extract text"))
    }

    #[test]
    fn test_request_5() {
        // yyyy-MM-dd HH-mm
        let path = "/unauth/fahrplanservice/v1/verbindungen/s/Zurich/s/Bern/ab/2019-09-20/10-14/";
        let mut resp = make_request(path).expect("Invalid request");

        println!("{}", resp.text().expect("Unable to extract text"))
    }

    #[test]
    fn test_request_train_formation() {
        // yyyy-MM-dd HH-mm
        let path = "/unauth/fahrplanservice/v1/trains/~H4sIAAAAAAAAAN1U227aQBD9FWvhrYB2bYNtKiTAygWladQkSm-qqoUMsMKs0e46aXP5n1bqM099Ij_WWUy4VSlVHgPCuzvMnJ0zMz63hHcnCoSOU2ngmyF1ErMyfe1SFtEIn9RzXwfMd8Iq9SilDv4o81x7DnBLSkRBL5XaqKxnRCpXOLNfs2m71TmeTY3dHh51ZtPzYqvBmieNTw-_legNncN280MjrPp4m9f82PADLwhZLWy-aSzua_IGc8PmIqwNSmJA4HsRc10bUIv8ECPygGAtYJ3A6oB_FTuxwxz8FIusOJsexbNp4WKvQAvxfoFRXFp2f4yPs058XmAF67N02js9QNNh563dx0cFn9lU7taWWvi4UPyGVXsKI_-O3ZVd5gd-EFU9_44WsHIaBmPAcpH651tykw1IXWZJUsItDHLjiBsYpErgiXRiDJF8nO-RA7bFWrLxGBSp46lErkBd88SAitNL6zfvFkOvTPSs6SKVpH5L-qkacyNALT0X5b5fer7lveGTrkHuurwNc_2Cg4AdNZlEGuQA5MOPKyi3Hn6qdJIqgykkQs552HSM4lJb-9EGwa97liOIAUjdG_K-ATkvw4L1KWjDMwy1cKMM1A0kaH5_iseJEghjOODosRLpcoWgCvo420MQaOzzRMN9aQm2z8ci-e7EKRJ1roUZOpOEf3e4Ar6Bvt_aQvf_B72daSFBa-cmleAI6TBtnF7Ctd4Ab396Dvi7TIDZgXz6LGSsMKgrbl9kZ5JqLboJbMJuoYZPo365x5kY2vnQBpJks5UrAVgN567JHKWpuhSSz6filiToZzLr4gcVqxtBgEOWysHCGlZyXcFI3u3zobKiZJWgTKOyS89pWPfcOqUVRH9Fcy3jcpTJvnl8EQcJKiNGeS5ZHFojkyGZ3Lbebz6UCEpp-cygII5gFy3U0B2UWFTzNhkFlVoYeTV_nVGe6BN5_531Rs6opv9R_WBX9WsVK8JudSvXXKT_ketGM6K6G24145FF9W8aVWKHqwv9FNQlKBQd3Zr392TvAr2vcpl7WbOGdHJJfhHte06h_n0jCs4fcP5_Ns4IAAA%3D/formation?tnr=25459&cicle=0&puic=85&abDatum=2019-09-20";
        let mut resp = make_request(path).expect("Invalid request");

        println!("{}", resp.text().expect("Unable to extract text"))
    }

    #[test]
    fn test_request_6() {
        let path = "/unauth/v1/verbindungen/STATION/8503000/STATION/8505300/{abAn}/{datum}/{zeit}/";
        let mut resp = make_request(path).expect("Invalid request");

        println!("{}", resp.text().expect("Unable to extract text"))
    }*/
}
