use crate::models::verbindung::Verbindung;
use std::fs;
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct VerbindungenResults {
    pub verbindungen: Vec<Verbindung>,

    #[serde(rename = "earlierUrl")]
    pub eralier_url: Option<String>,

    #[serde(rename = "lateUrl")]
    pub late_url: Option<String>,

    #[serde(rename = "verbindungPreisUrl")]
    pub verbindung_preis_url: String
}

#[test]
fn parse_verbindungen_1(){
    let f = fs::read("./resources/test/verbindungen-1.json")
                        .expect("File not found");

    let vr : VerbindungenResults = serde_json::from_str(
            std::str::from_utf8(&f)
                .expect("Unable to parse file into string"))
        .expect("Unable to decode from JSON");

    assert_gt!(vr.verbindungen.len(), 0);
}

#[test]
fn parse_verbindungen_2(){
    let f = fs::read("./resources/test/verbindungen-2.json")
        .expect("File not found");

    let vr : VerbindungenResults = serde_json::from_str(
        std::str::from_utf8(&f)
            .expect("Unable to parse file into string"))
        .expect("Unable to decode from JSON");

    assert_gt!(vr.verbindungen.len(), 0);
}