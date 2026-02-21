use serde::{Deserialize, Serialize};
use std::fmt;

/// Transport designation as returned by the new timetable API.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransportDesignation {
    pub vehicle_icon: Option<String>,
    pub transport_insignia_icon: Option<String>,
    pub transport_display_name: String,
    pub transport_extra_info: Option<String>,
}

impl fmt::Display for TransportDesignation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.transport_extra_info {
            Some(extra) if !extra.is_empty() => {
                write!(f, "{} {}", self.transport_display_name, extra)
            }
            _ => write!(f, "{}", self.transport_display_name),
        }
    }
}
