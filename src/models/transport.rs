use core::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct TransportBezeichnung {
    #[serde(rename = "oevIcon")]
    pub oev_icon : String,

    #[serde(rename = "transportDirection")]
    pub transport_direction : String,

    #[serde(rename = "transportIcon")]
    pub transport_icon : String,

    #[serde(rename = "transportIconSuffix")]
    pub transport_icon_suffix : Option<String>,

    #[serde(rename = "transportLabel")]
    pub transport_label : String,

    #[serde(rename = "transportLabelBgColor")]
    pub transport_label_bg_color : Option<String>,

    #[serde(rename = "transportLabelTextColor")]
    pub transport_label_text_color : Option<String>,

    #[serde(rename = "transportName")]
    pub transport_name : Option<String>,

    #[serde(rename = "transportText")]
    pub transport_text : String,
}

impl fmt::Display for TransportBezeichnung {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {}", self.transport_label, self.transport_text)
    }
}