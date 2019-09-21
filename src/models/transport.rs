#[derive(Serialize, Deserialize)]
pub struct TransportBezeichnung {
    pub oevIcon : String,
    pub transportDirection : String,
    pub transportIcon : String,
    pub transportIconSuffix : String,
    pub transportLabel : String,
    pub transportLabelBgColor : Option<String>,
    pub transportLabelTextColor : Option<String>,
    pub transportName : Option<String>,
    pub transportText : String,
}