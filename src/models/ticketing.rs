#[derive(Serialize, Deserialize, Debug)]
pub struct TicketingInfo {
    #[serde(rename="buttonText")]
    pub button_text: Option<String>,

    #[serde(rename="dialogMessage")]
    pub dialog_message: Option<String>,

    #[serde(rename="dialogTitle")]
    pub dialog_title: Option<String>,

    #[serde(rename="isAvailable")]
    pub is_available: bool,
}