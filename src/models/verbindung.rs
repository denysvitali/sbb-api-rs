use serde::{Serialize, Deserialize};
use crate::models::koordinaten::Koordinaten;
use crate::models::transport::TransportBezeichnung;
use crate::models::realtime_info::{SectionRealtimeInfo, RealtimeInfo};
use crate::models::legend::{LegendOccupancy, LegendItem};
use crate::models::ticketing::TicketingInfo;

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
    pub verbindung_sections: Vec<VerbindungSections>,
 
   #[serde(rename = "verkehrstage")]
    pub verkehrstage: Vec<String>,
 
   #[serde(rename = "vias")]
    pub vias: Option<Vec<String>>,
 
   #[serde(rename = "zuschlagspflicht")]
    pub zuschlagspflicht: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VerbindungSections {
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