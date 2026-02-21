
use chrono::Local;
use openssl::hash::MessageDigest;
use openssl::sign::Signer;
use openssl::pkey::PKey;
use uuid::Uuid;

/// HMAC key for the vnext API (from SBB Android app)
const HMAC_KEY: &str = r#"GY>b+.[0]S@b~f!2;4MU&GK<xQpO#;mG>"VuxE^,nh~Ev6!_cr\[rL'zL5<qX'D]"#;

/// Generate a random UUID for the X-APP-TOKEN header.
pub fn generate_app_token() -> String {
    Uuid::new_v4().to_string()
}

/// Get current date in YYYY-MM-DD format for X-API-DATE header.
pub fn get_date() -> String {
    Local::now().format("%Y-%m-%d").to_string()
}

/// Compute HMAC-SHA1 signature for the API authorization.
/// Data: path + date, Key: HMAC_KEY, Base64 encoded result.
pub fn get_authorization(path: &str, date: &str) -> String {
    let key = PKey::hmac(HMAC_KEY.as_bytes()).expect("Invalid HMAC key");
    let mut signer = Signer::new(MessageDigest::sha1(), &key).expect("Unable to create signer");

    let data = format!("{}{}", path, date);
    signer.update(data.as_bytes()).expect("Failed to update HMAC");

    base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &signer.sign_to_vec().expect("HMAC sign failed"))
}
