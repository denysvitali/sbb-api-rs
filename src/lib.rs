pub mod connections;
pub mod authenticator;
pub mod models;

use reqwest::{Certificate, Method, Url, Response};
use reqwest::header::{HeaderMap, USER_AGENT, HeaderValue, HeaderName};
use std::str::FromStr;

pub const API_ENDPOINT: &str = "https://active.vnext.app.sbb.ch";
pub const SBB_UA: &str = "SBBmobile/12.49.5.166.master Android/14 (Google;Pixel 8;android14)";

/// SBB's self-signed root CA (*.sbbmobile.ch). Required to verify the API server certificate.
const SBB_CA_CERT: &[u8] = include_bytes!("../resources/ca_cert.crt");

pub(crate) async fn make_request(url: Url, path: &str) -> Result<Response, reqwest::Error> {
    let mut headers = HeaderMap::new();
    headers.append(
        USER_AGENT,
        HeaderValue::from_str(SBB_UA).expect("Unable to parse User-Agent"),
    );
    headers.append(
        HeaderName::from_str("USE-CASE").expect("Invalid header name"),
        HeaderValue::from_static("TIMETABLE"),
    );

    // App token - random UUID
    let app_token = authenticator::generate_app_token();
    headers.append(
        HeaderName::from_str("X-APP-TOKEN").expect("Invalid header name"),
        HeaderValue::from_str(&app_token).expect("Invalid header value"),
    );

    // API auth - HMAC-SHA1(path + date)
    let date = authenticator::get_date();
    let auth = authenticator::get_authorization(path, &date);
    headers.append(
        HeaderName::from_str("X-API-AUTHORIZATION").expect("Invalid header name"),
        HeaderValue::from_str(&auth).expect("Invalid header value"),
    );
    headers.append(
        HeaderName::from_str("X-API-DATE").expect("Invalid header name"),
        HeaderValue::from_str(&date).expect("Invalid header value"),
    );

    let ca = Certificate::from_der(SBB_CA_CERT).expect("Unable to parse SBB CA certificate");

    let client = reqwest::Client::builder()
        .add_root_certificate(ca)
        .default_headers(headers)
        .build()
        .expect("Unable to create client");

    let request = reqwest::Request::new(Method::GET, url);
    client.execute(request).await
}
