#[derive(Serialize, Deserialize)]
pub struct SectionRealtimeInfo {
    pub abfahrtCancellation : bool,
    pub abfahrtDelayUndefined : bool,
    pub abfahrtIstDatum : String,
    pub abfahrtIstZeit : String,
    pub abfahrtPlatformChange : bool,
    pub ankunftCancellation : bool,
    pub ankunftDelayUndefined : bool,
    pub ankunftIstDatum : String,
    pub ankunftIstZeit : String,
    pub ankunftPlatformChange : bool,
}


#[derive(Serialize, Deserialize)]
pub struct RealtimeInfo {
    pub abfahrtIstDatum : String,
    pub abfahrtIstZeit : String,
    pub alternativeMsg : Option<String>,
    pub ankunftIstDatum : String,
    pub ankunftIstZeit : String,
    pub cancellationMsg : Option<String>,
    pub detailMsg : Option<String>,
    pub icon : Option<String>,
    pub isAlternative : bool,
    pub nextAlternative : Option<String>,
    pub platformChange : Option<String>,
}