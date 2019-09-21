use crate::models::verbindung::Verbindung;
use std::fs;
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize)]
pub struct VerbindungenResults {
    pub verbindungen: Vec<Verbindung>,
    pub earlierUrl: String,
    pub lateUrl: String,
    pub verbindungPreisUrl: String
}

#[test]
fn parse_verbindungen(){
    let f = fs::read("./resources/test/verbindungen-1.json")
                        .expect("File not found");

    let vr : VerbindungenResults = serde_json::from_str(
            std::str::from_utf8(&f)
                .expect("Unable to parse file into string"))
        .expect("Unable to decode from JSON");

    assert_gt!(0, vr.verbindungen.len());
}