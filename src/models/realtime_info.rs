#[derive(Serialize, Deserialize, Debug)]
pub struct SectionRealtimeInfo {
    #[serde(rename = "abfahrtCancellation")]
    pub abfahrt_cancellation: bool,

    #[serde(rename = "abfahrtDelayUndefined")]
    pub abfahrt_delay_undefined: bool,

    #[serde(rename = "abfahrtIstDatum")]
    pub abfahrt_ist_datum: String,

    #[serde(rename = "abfahrtIstZeit")]
    pub abfahrt_ist_zeit: String,

    #[serde(rename = "abfahrtPlatformChange")]
    pub abfahrt_platform_change: bool,

    #[serde(rename = "ankunftCancellation")]
    pub ankunft_cancellation: bool,

    #[serde(rename = "ankunftDelayUndefined")]
    pub ankunft_delay_undefined: bool,

    #[serde(rename = "ankunftIstDatum")]
    pub ankunft_ist_datum: String,

    #[serde(rename = "ankunftIstZeit")]
    pub ankunft_ist_zeit: String,

    #[serde(rename = "ankunftPlatformChange")]
    pub ankunft_platform_change: bool,

}


#[derive(Serialize, Deserialize, Debug)]
pub struct RealtimeInfo {
    #[serde(rename = "abfahrtIstDatum")]
    pub abfahrt_ist_datum: String,

    #[serde(rename = "abfahrtIstZeit")]
    pub abfahrt_ist_zeit: String,

    #[serde(rename = "alternativeMsg")]
    pub alternative_msg: Option<String>,

    #[serde(rename = "ankunftIstDatum")]
    pub ankunft_ist_datum: String,

    #[serde(rename = "ankunftIstZeit")]
    pub ankunft_ist_zeit: String,

    #[serde(rename = "cancellationMsg")]
    pub cancellation_msg: Option<String>,

    #[serde(rename = "detailMsg")]
    pub detail_msg: Option<String>,

    #[serde(rename = "icon")]
    pub icon: Option<String>,

    #[serde(rename = "isAlternative")]
    pub is_alternative: bool,

    #[serde(rename = "nextAlternative")]
    pub next_alternative: Option<String>,

    #[serde(rename = "platformChange")]
    pub platform_change: Option<String>,
}