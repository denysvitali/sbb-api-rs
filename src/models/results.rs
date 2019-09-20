use crate::models::verbindung::Verbindung;

#[derive(Serialize, Deserialize)]
pub struct VerbindungenResults {
    pub verbindungen: Vec<Verbindung>,
    pub earlierUrl: String,
    pub lateUrl: String,
    pub verbindungPreisUrl: String
}