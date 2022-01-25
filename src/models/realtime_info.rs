#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SectionRealtimeInfo {
    pub abfahrt_cancellation: bool,
    pub abfahrt_delay_undefined: bool,
    pub abfahrt_ist_datum: Option<String>,
    pub abfahrt_ist_zeit: Option<String>,
    pub abfahrt_platform_change: bool,
    pub ankunft_cancellation: bool,
    pub ankunft_delay_undefined: bool,
    pub ankunft_ist_datum: Option<String>,
    pub ankunft_ist_zeit: Option<String>,
    pub ankunft_platform_change: bool,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct RealtimeInfo {
    pub abfahrt_ist_datum: Option<String>,
    pub abfahrt_ist_zeit: Option<String>,
    pub alternative_msg: Option<String>,
    pub ankunft_ist_datum: Option<String>,
    pub ankunft_ist_zeit: Option<String>,
    pub cancellation_msg: Option<String>,
    pub detail_msg: Option<String>,
    pub icon: Option<String>,
    pub is_alternative: Option<bool>,
    pub next_alternative: Option<String>,
    pub platform_change: Option<String>,
}