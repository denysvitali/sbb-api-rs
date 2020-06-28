use openssl::x509::X509;
use openssl::hash::{MessageDigest, DigestBytes, Hasher};
use openssl::sign::Signer;
use openssl::pkey::PKey;
static KEY : &str = "c3eAd3eC3a7845dE98f73942b3d5f9c0";


pub fn get_authorization<'a>(path: &str, date: &str) -> String {
    let hmac_key = get_key();
    let key = PKey::hmac(hmac_key.as_bytes()).unwrap();
    let mut signer = Signer::new(MessageDigest::sha1(), &key).unwrap();

    let digest = format!("{}{}", path, date);
    let _ = signer.update(digest.as_bytes());

    base64::encode(&signer.sign_to_vec().expect("Unable to sign"))
}

pub fn get_certificate_hash() -> String {
    let cert = X509::from_der(
        include_bytes!("../resources/ca_cert.crt")
    ).expect("Invalid CA certificate");

    let r : DigestBytes = cert.digest(MessageDigest::sha1()).expect("Digest");
    base64::encode(&r)
}

pub fn retrieve_key(cert_hash: &str, key: &str) -> String {
    let mut hasher : Hasher = Hasher::new(MessageDigest::sha256()).expect("Unable to initialize hasher");

    let cleartext = format!("{}{}", cert_hash, key);
    hasher.update(cleartext.as_bytes()).expect("Failed to update hasher");
    let bytes = hasher.finish().expect("Unable to hash!");
    hex::encode(bytes)
}

pub fn get_key() -> String {
    retrieve_key(&get_certificate_hash(), KEY)
}

#[cfg(test)]
mod test {
    use crate::authenticator::{get_authorization, get_certificate_hash};

    #[test]
    fn url_1(){
        let date = "2020-06-28";
        let auth = "hL89gUidDebOUNUCP/+5vbj+0Iw=";
        let path = "/unauth/fahrplanservice/v1/verbindungen/s/Z%25C3%25BCrich%2520HB/s/Bern/ab/2019-09-20/21-14/";

        assert_eq!(get_authorization(path, date), auth);
    }

    #[test]
    fn test_certificate_hash(){
        assert_eq!("WdfnzdQugRFUF5b812hZl3lAahM=", get_certificate_hash());
    }

}

