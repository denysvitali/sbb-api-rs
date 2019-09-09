extern crate openssl;
extern crate hex;
pub mod authenticator;

#[cfg(test)]
mod tests {
    use super::authenticator;
    #[test]
    fn test_path_1() {
        let path = "/unauth/ticketingservice/zvs/v0/features";
        assert_eq!("WdfnzdQugRFUF5b812hZl3lAahM=", authenticator::get_certificate_hash());

        assert_eq!(authenticator::get_authorization(path, "2019-09-05"), "wqhPBCfC9oc8gp62FVVIiNIADzg=");
    }

    #[test]
    fn test_path_2() {
        let path = "/unauth/ticketingservice/zvs/v0/ghettobox/";
        assert_eq!(authenticator::get_authorization(path, "2019-09-05"), "3fgUyXQoMieevNYULWbo3OPsd4w=");
    }
}
