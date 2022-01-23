use serde::{Serialize, Deserialize};
use crate::models::koordinaten::Koordinaten;
use crate::models::transport::TransportBezeichnung;
use crate::models::realtime_info::{SectionRealtimeInfo, RealtimeInfo};
use crate::models::legend::{LegendOccupancy, LegendItem};
use crate::models::ticketing::TicketingInfo;
use core::fmt;
use std::time::Duration;
use std::str::FromStr;

use mockall::predicate::*;
use regex::bytes::Regex;
use regex::Match;

#[derive(Serialize, Deserialize, Debug)]
pub struct Verbindung {
   #[serde(rename = "abfahrt")]
    pub abfahrt: String,
 
   #[serde(rename = "abfahrtDate")]
    pub abfahrt_date: String,
 
   #[serde(rename = "abfahrtGleis")]
    pub abfahrt_gleis: String,
 
   #[serde(rename = "abfahrtTime")]
    pub abfahrt_time: String,
 
   #[serde(rename = "angeboteUrl")]
    pub angebote_url: String,
 
   #[serde(rename = "ankunft")]
    pub ankunft: String,
 
   #[serde(rename = "ankunftDate")]
    pub ankunft_date: String,
 
   #[serde(rename = "ankunftTime")]
    pub ankunft_time: String,
 
   #[serde(rename = "belegungErste")]
    pub belegung_erste: String,
 
   #[serde(rename = "belegungZweite")]
    pub belegung_zweite: String,
 
   #[serde(rename = "dayDifference")]
    pub day_difference: String,
 
   #[serde(rename = "dayDifferenceAccessibility")]
    pub day_difference_accessibility: String,
 
   #[serde(rename = "departureTrackLabel")]
    pub departure_track_label: String,
 
   #[serde(rename = "departureTrackLabelAccessibility")]
    pub departure_track_label_accessibility: String,
 
   #[serde(rename = "duration")]
    pub duration: String,
 
   #[serde(rename = "durationAccessibility")]
    pub duration_accessibility: String,
 
   #[serde(rename = "isInternationalVerbindung")]
    pub is_international_verbindung: bool,
 
   #[serde(rename = "legendBfrItems")]
    pub legend_bfr_items: Vec<String>,
 
   #[serde(rename = "legendItems")]
    pub legend_items: Vec<LegendItem>,
 
   #[serde(rename = "legendOccupancyItems")]
    pub legend_occupancy_items: Vec<LegendOccupancy>,
 
   #[serde(rename = "realtimeInfo")]
    pub realtime_info: RealtimeInfo,
 
   #[serde(rename = "reconstructionContext")]
    pub reconstruction_context: String,
 
   #[serde(rename = "serviceAttributes")]
    pub service_attributes: Vec<String>,
 
   #[serde(rename = "ticketingInfo")]
    pub ticketing_info: TicketingInfo,
 
   #[serde(rename = "transfers")]
    pub transfers: i32,
 
   #[serde(rename = "transportBezeichnung")]
    pub transport_bezeichnung: TransportBezeichnung,
 
   #[serde(rename = "verbindungAbpreisContext")]
    pub verbindung_abpreis_context: String,
 
   #[serde(rename = "verbindungId")]
    pub verbindung_id: String,
 
   #[serde(rename = "verbindungSections")]
    pub verbindung_sections: Vec<VerbindungSection>,
 
   #[serde(rename = "verkehrstage")]
    pub verkehrstage: Vec<String>,
 
   #[serde(rename = "vias")]
    pub vias: Option<Vec<String>>,
 
   #[serde(rename = "zuschlagspflicht")]
    pub zuschlagspflicht: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VerbindungSection {
    #[serde(rename = "abfahrtCancellation")]
    pub abfahrt_cancellation: bool,

    #[serde(rename = "abfahrtDatum")]
    pub abfahrt_datum: String,

    #[serde(rename = "abfahrtGleis")]
    pub abfahrt_gleis: String,

    #[serde(rename = "abfahrtKoordinaten")]
    pub abfahrt_koordinaten: Koordinaten,

    #[serde(rename = "abfahrtName")]
    pub abfahrt_name: String,

    #[serde(rename = "abfahrtPlatformChange")]
    pub abfahrt_platform_change: bool,

    #[serde(rename = "abfahrtTime")]
    pub abfahrt_time: String,

    #[serde(rename = "actionUrl")]
    pub action_url: Option<String>,

    #[serde(rename = "ankunftCancellation")]
    pub ankunft_cancellation: bool,

    #[serde(rename = "ankunftDatum")]
    pub ankunft_datum: String,

    #[serde(rename = "ankunftGleis")]
    pub ankunft_gleis: String,

    #[serde(rename = "ankunftKoordinaten")]
    pub ankunft_koordinaten: Koordinaten,

    #[serde(rename = "ankunftName")]
    pub ankunft_name: String,

    #[serde(rename = "ankunftPlatformChange")]
    pub ankunft_platform_change: bool,

    #[serde(rename = "ankunftTime")]
    pub ankunft_time: String,

    #[serde(rename = "arrivalTrackLabel")]
    pub arrival_track_label: Option<String>,

    #[serde(rename = "arrivalTrackLabelAccessibility")]
    pub arrival_track_label_accessibility: String,

    #[serde(rename = "belegungErste")]
    pub belegung_erste: String,

    #[serde(rename = "belegungZweite")]
    pub belegung_zweite: String,

    #[serde(rename = "departureTrackLabel")]
    pub departure_track_label: Option<String>,

    #[serde(rename = "departureTrackLabelAccessibility")]
    pub departure_track_label_accessibility: String,

    #[serde(rename = "durationProzent")]
    pub duration_prozent: Option<String>,

    #[serde(rename = "formationUrl")]
    pub formation_url: Option<String>,

    #[serde(rename = "previewType")]
    pub preview_type: String,

    #[serde(rename = "realtimeInfo")]
    pub realtime_info: SectionRealtimeInfo,

    #[serde(rename = "transportBezeichnung")]
    pub transport_bezeichnung: Option<TransportBezeichnung>,

    #[serde(rename = "transportHinweis")]
    pub transport_hinweis: Option<String>,

    #[serde(rename = "transportServiceAttributes")]
    pub transport_service_attributes: Vec<String>,

    #[serde(rename = "type")]
    pub verbindung_type: String,
}

impl fmt::Display for Verbindung {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut verbindungen= String::new();
        for vs in self.verbindung_sections.as_slice() {
            verbindungen = format!("{}, {}", verbindungen, vs)
        }

        write!(f, "{} ({}): {}", self.transport_bezeichnung, self.duration, verbindungen)
    }
}

impl fmt::Display for VerbindungSection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} - {} {}",
                match &self.transport_bezeichnung {
                    Some(t) => format!("{}", t),
                    None => String::new()
                },
                self.abfahrt_name,
                self.abfahrt_time,
                self.ankunft_name,
                self.ankunft_time)
    }
}

fn cap_to_u64(m: Option<regex::bytes::Match>) -> u64 {
    if m.is_none() {
        return 0;
    }

    let s = std::str::from_utf8(m.unwrap().as_bytes()).unwrap_or(&"");
    return u64::from_str(s).unwrap_or(0);
}

impl Verbindung {
    pub fn duration(&self) -> Duration {
        let duration = &self.duration;
        let re = Regex::new(r"^(?:(\d+) h|)(?: |)(?:(\d+) min|)$").unwrap();

        if !re.is_match(duration.as_bytes()) {
            return Duration::from_millis(0)
        }

        for cap in re.captures_iter(duration.as_bytes()) {
            println!("cap={:?}", cap);
            let hours = cap_to_u64(cap.get(1));
            let minutes = cap_to_u64(cap.get(2));


            return Duration::from_secs(hours*60*60 + minutes*60);
        }

        return Duration::from_millis(1)
    }
}

impl AsRef<Verbindung> for Verbindung {
    fn as_ref(&self) -> &Verbindung {
        return self
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use std::time::Duration;
    use crate::models::verbindung::Verbindung;

    #[test]
    fn test_verbindung_duration() {
        let f = fs::read("./resources/test/verbindung-1.json")
            .expect("File not found");
    
        let vr : Verbindung = serde_json::from_str(
            std::str::from_utf8(&f)
            .expect("Unable to parse file into string"))
            .expect("Unable to decode from JSON");
    
        assert_eq!(Duration::from_secs(56 * 60), vr.duration())
    }
    
    #[test]
    fn test_verbindung_duration_2() {
        let f = fs::read("./resources/test/verbindung-2.json")
            .expect("File not found");
    
        let vr : Verbindung = serde_json::from_str(
            std::str::from_utf8(&f)
            .expect("Unable to parse file into string"))
            .expect("Unable to decode from JSON");
    
        assert_eq!(Duration::from_secs(1 * 60 * 60 + 5 * 60), vr.duration())
    }
}
