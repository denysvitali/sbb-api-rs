use std::fmt::Formatter;

use serde::{Deserialize, Serialize};
use crate::models::verbindung::Verbindung;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VerbindungenResults {
    pub verbindungen: Vec<Verbindung>,
    pub earlier_url: Option<String>,
    pub late_url: Option<String>,
    pub verbindung_preis_url: String,
}

impl std::fmt::Display for VerbindungenResults {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "VerbindungenResults{{\n")?;
        for verb in &self.verbindungen {
            write!(f, "{},\n", verb)?;
        }
        write!(f, "earlier={:?}, late={:?}, preis={}",
               self.earlier_url,
               self.late_url,
               self.verbindung_preis_url
        )
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::models::results::VerbindungenResults;

    #[test]
    fn parse_verbindungen_1() {
        let f = fs::read("./resources/test/verbindungen-1.json")
            .expect("File not found");

        let vr: VerbindungenResults = serde_json::from_str(
            std::str::from_utf8(&f)
                .expect("Unable to parse file into string"))
            .expect("Unable to decode from JSON");

        assert_gt!(vr.verbindungen.len(), 0);
    }

    #[test]
    fn parse_verbindungen_2() {
        let f = fs::read("./resources/test/verbindungen-2.json")
            .expect("File not found");

        let vr: VerbindungenResults = serde_json::from_str(
            std::str::from_utf8(&f)
                .expect("Unable to parse file into string"))
            .expect("Unable to decode from JSON");

        assert_gt!(vr.verbindungen.len(), 0);
    }

    #[test]
    fn parse_verbindungen_3() {
        let f = fs::read("./resources/test/verbindungen-3.json")
            .expect("File not found");

        let vr: VerbindungenResults = serde_json::from_str(
            std::str::from_utf8(&f)
                .expect("Unable to parse file into string"))
            .expect("Unable to decode from JSON");

        assert_gt!(vr.verbindungen.len(), 0);

        for v in vr.verbindungen {
            println!("{}", v);
        }
    }

    #[test]
    fn parse_verbindungen_4() {
        let f = fs::read("./resources/test/verbindungen-4.json")
            .expect("File not found");

        let vr: VerbindungenResults = serde_json::from_str(
            std::str::from_utf8(&f)
                .expect("Unable to parse file into string"))
            .expect("Unable to decode from JSON");

        assert_gt!(vr.verbindungen.len(), 0);

        for v in vr.verbindungen {
            println!("{}", v);
        }
    }

    #[test]
    fn parse_verbindungen_5() {
        let f = fs::read("./resources/test/verbindungen-5.json")
            .expect("File not found");

        let vr: VerbindungenResults = serde_json::from_str(
            std::str::from_utf8(&f)
                .expect("Unable to parse file into string"))
            .expect("Unable to decode from JSON");

        assert_gt!(vr.verbindungen.len(), 0);
        assert_eq!(vr.verbindungen[0].duration().as_secs(), (2 * 60 + 47) * 60);

        for v in vr.verbindungen {
            println!("{:?} {}", v.as_ref().duration(), v);
        }
    }
}
