use serde::{Deserialize, Serialize};
use crate::models::transport::TransportDesignation;

/// Top-level response from `GET /api/timetable/v2/trips`.
/// Corresponds to `TripSearchResponseDto` in the Android app.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TripSearchResponse {
    pub trips: Vec<Trip>,
    /// Cursor to fetch earlier connections (pass as `pagingCursor`).
    pub earlier_paging_cursor: Option<String>,
    /// Cursor to fetch later connections (pass as `pagingCursor`).
    pub later_paging_cursor: Option<String>,
}

/// A single connection result. Corresponds to `TripDto`.
#[derive(Serialize, Deserialize, Debug)]
pub struct Trip {
    pub meta: TripMeta,
    pub summary: TripSummary,
    /// Leg-by-leg detail. Present in list responses; may be absent for some trips.
    pub detail: Option<TripDetail>,
}

impl Trip {
    /// Number of transfers (PT legs minus 1, minimum 0).
    pub fn transfers(&self) -> usize {
        let pt_legs = self
            .detail
            .as_ref()
            .map(|d| d.legs.iter().filter(|l| matches!(l, TripLeg::PtRideLeg(_))).count())
            .unwrap_or(1);
        pt_legs.saturating_sub(1)
    }
}

/// Metadata for a trip. Corresponds to `TripMetaDto`.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TripMeta {
    pub id: String,
    pub next_refresh: Option<i64>,
}

/// Summary card data shown in the connection list. Corresponds to `TripSummaryDto`.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TripSummary {
    pub duration: Option<TripDuration>,
    /// Occupancy level for first class: "LOW", "MEDIUM", "HIGH", or absent.
    pub occupancy_first_class_max: Option<String>,
    /// Occupancy level for second class.
    pub occupancy_second_class_max: Option<String>,
    pub departure_display_name: String,
    pub arrival_display_name: String,
    pub departure_anchor: DepartureAnchor,
    pub arrival_anchor: ArrivalAnchor,
}

/// Human-readable duration. Corresponds to `DurationDto`.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TripDuration {
    /// Localized label, e.g. "56 min" or "1 h 05 min".
    pub localized_label: String,
    pub duration_in_minutes: i32,
}

/// Departure end of the connection summary. Corresponds to `TripSummaryDepartureAnchorDto`.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DepartureAnchor {
    pub place_name: String,
    /// Scheduled departure time (ISO 8601 local datetime string).
    pub time_aimed: Option<String>,
    /// Expected (real-time) departure time.
    pub time_expected: Option<String>,
    /// Pre-formatted time string for display (e.g. "10:32").
    pub display_time: String,
    /// Pre-formatted date string for display (e.g. "Fr, 21.02.").
    pub display_date: String,
    pub quay: Option<Quay>,
    pub direction: Option<String>,
    pub transport_designation: Option<TransportDesignation>,
}

/// Arrival end of the connection summary. Corresponds to `TripSummaryArrivalAnchorDto`.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ArrivalAnchor {
    pub place_name: String,
    pub time_aimed: Option<String>,
    pub time_expected: Option<String>,
    pub display_time: String,
    pub display_date: String,
    pub quay: Option<Quay>,
}

/// Platform / track information. Corresponds to `QuayDto`.
#[derive(Serialize, Deserialize, Debug)]
pub struct Quay {
    pub name: String,
    pub changed: bool,
}

/// Leg-level detail for a trip. Corresponds to `TripDetailListDto`.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TripDetail {
    pub legs: Vec<TripLeg>,
    pub rt_pt_ride_leg_info: Option<RtInfo>,
}

/// A polymorphic leg within a trip. The `type` field selects the variant.
/// Corresponds to the sealed `TripLegDto` hierarchy in the Android app.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum TripLeg {
    /// Public-transport ride leg (type = "PtRideLeg").
    PtRideLeg(PtRideLeg),
    /// Walking / access leg (type = "AccessLeg").
    AccessLeg(AccessLeg),
    /// Transfer / change leg (type = "ChangeLeg").
    ChangeLeg(ChangeLeg),
}

/// A public-transport ride segment. Corresponds to `TripLegDto$PtRideLeg`.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PtRideLeg {
    pub direction: Option<String>,
    pub marketing_name: Option<String>,
    pub first_transport_designation: Option<TransportDesignation>,
    pub departure_stop_point: StopPoint,
    pub arrival_stop_point: StopPoint,
    pub rt_pt_ride_leg_info: Option<RtInfo>,
}

/// A walking or access segment.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccessLeg {
    pub departure_stop_point: Option<StopPoint>,
    pub arrival_stop_point: Option<StopPoint>,
}

/// A transfer / change segment between two PT rides.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChangeLeg {
    pub departure_stop_point: Option<StopPoint>,
    pub arrival_stop_point: Option<StopPoint>,
}

/// A stop within a leg. Corresponds to `ScheduledStopPointDto`.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StopPoint {
    pub display_name: String,
    pub occupancy_first_class: Option<String>,
    pub occupancy_second_class: Option<String>,
    pub arrival_time: Option<StopTime>,
    pub departure_time: Option<StopTime>,
    pub quay: Option<Quay>,
    pub rt_stop_info: Option<RtInfo>,
}

/// Scheduled and real-time times at a stop. Corresponds to `StopTimeDto`.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StopTime {
    /// Scheduled time (ISO 8601 local datetime).
    pub time_aimed: Option<String>,
    /// Real-time expected time.
    pub time_expected: Option<String>,
    /// Pre-formatted time string for display (e.g. "10:32").
    pub display_time: Option<String>,
}

/// Real-time disruption info. Corresponds to `RtInfoDto`.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RtInfo {
    /// Type of real-time event: "DELAY", "CANCELLED", "PLATFORM_CHANGE", etc.
    pub rt_type: Option<String>,
    pub display_name: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bern_basel() {
        // This test file is Basel SBB -> Bern (arrival search at 15:00)
        let data = std::fs::read("./resources/test/sbb_api_response_0.json").unwrap();
        let resp: TripSearchResponse = serde_json::from_slice(&data).unwrap();
        assert!(!resp.trips.is_empty());

        let trip = &resp.trips[0];
        assert_eq!(trip.summary.departure_display_name, "Basel SBB");
        assert_eq!(trip.summary.arrival_display_name, "Bern");

        // Check summary has duration
        let duration = trip.summary.duration.as_ref().expect("duration missing");
        assert!(duration.duration_in_minutes > 0);

        // Check detail has legs (this trip has transfers)
        let detail = trip.detail.as_ref().expect("detail missing");
        assert!(detail.legs.len() >= 2);

        // First leg should be PtRideLeg starting at Basel SBB
        match &detail.legs[0] {
            TripLeg::PtRideLeg(pt) => {
                assert_eq!(pt.departure_stop_point.display_name, "Basel SBB");
            }
            _ => panic!("Expected PtRideLeg"),
        }

        // Last leg should end at Bern
        let last = detail.legs.last().unwrap();
        match last {
            TripLeg::PtRideLeg(pt) => {
                assert_eq!(pt.arrival_stop_point.display_name, "Bern");
            }
            _ => panic!("Expected PtRideLeg"),
        }
    }

    #[test]
    fn test_parse_basel_bern_arrival() {
        let data = std::fs::read("./resources/test/sbb_api_response_1.json").unwrap();
        let resp: TripSearchResponse = serde_json::from_slice(&data).unwrap();
        assert!(!resp.trips.is_empty());

        let trip = &resp.trips[0];
        assert_eq!(trip.summary.departure_display_name, "Basel SBB");
        assert_eq!(trip.summary.arrival_display_name, "Bern");
    }

    #[test]
    fn test_parse_geneve_zurich() {
        let data = std::fs::read("./resources/test/sbb_api_response_2.json").unwrap();
        let resp: TripSearchResponse = serde_json::from_slice(&data).unwrap();
        assert!(!resp.trips.is_empty());

        let trip = &resp.trips[0];
        // Should have transfers
        assert!(trip.transfers() > 0);

        let detail = trip.detail.as_ref().expect("detail missing");
        // Count leg types
        let pt_count = detail.legs.iter().filter(|l| matches!(l, TripLeg::PtRideLeg(_))).count();
        assert!(pt_count >= 2);
    }

    #[test]
    fn test_parse_zurich_chur() {
        let data = std::fs::read("./resources/test/sbb_api_response_3.json").unwrap();
        let resp: TripSearchResponse = serde_json::from_slice(&data).unwrap();
        assert!(!resp.trips.is_empty());

        let trip = &resp.trips[0];
        assert_eq!(trip.summary.departure_display_name, "Zürich HB");
        assert_eq!(trip.summary.arrival_display_name, "Chur");

        // Should be direct (no transfers)
        assert_eq!(trip.transfers(), 0);

        let detail = trip.detail.as_ref().expect("detail missing");
        assert_eq!(detail.legs.len(), 1);
    }

    #[test]
    fn test_occupancy_parsing() {
        let data = std::fs::read("./resources/test/sbb_api_response_0.json").unwrap();
        let resp: TripSearchResponse = serde_json::from_slice(&data).unwrap();

        let trip = &resp.trips[0];
        assert_eq!(trip.summary.occupancy_first_class_max.as_deref(), Some("LOW"));
        assert_eq!(trip.summary.occupancy_second_class_max.as_deref(), Some("LOW"));
    }

    #[test]
    fn test_paging_cursors() {
        let data = std::fs::read("./resources/test/sbb_api_response_0.json").unwrap();
        let resp: TripSearchResponse = serde_json::from_slice(&data).unwrap();

        assert!(resp.earlier_paging_cursor.is_some());
        assert!(resp.later_paging_cursor.is_some());
    }
}
