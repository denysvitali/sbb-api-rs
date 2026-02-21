use clap::Parser;
use chrono::{Local, NaiveDate, NaiveTime, TimeZone, Utc};
use sbb_api::connections::get_connections;
use sbb_api::models::location::SearchDateTimeType;
use sbb_api::models::trip::TripLeg;
use std::time::Duration;
use tokio::time::timeout;

#[derive(Parser)]
#[command(name = "sbb", about = "Query SBB train connections")]
struct Cli {
    /// Departure station or address
    from: String,
    /// Arrival station or address
    to: String,
    /// UIC reference for departure (e.g. 8503000 for Zürich HB)
    #[arg(long = "from-ref", value_name = "UIC")]
    from_ref: Option<String>,
    /// UIC reference for arrival
    #[arg(long = "to-ref", value_name = "UIC")]
    to_ref: Option<String>,
    /// Departure time (HH:MM)
    #[arg(long = "at", value_name = "HH:MM")]
    at: Option<String>,
    /// Departure date (YYYY-MM-DD)
    #[arg(long = "date", value_name = "YYYY-MM-DD")]
    date: Option<String>,
    /// Search for connections arriving at the given time instead of departing
    #[arg(long = "arrival")]
    arrival: bool,
    /// Print debug information to stderr
    #[arg(short = 'd', long = "debug")]
    debug: bool,
}

macro_rules! debug {
    ($enabled:expr, $($arg:tt)*) => {
        if $enabled {
            eprintln!("[debug] {}", format!($($arg)*));
        }
    };
}

#[actix_rt::main]
async fn main() {
    let cli = Cli::parse();
    let dbg = cli.debug;

    let now = Local::now();

    let date = match &cli.date {
        Some(d) => NaiveDate::parse_from_str(d, "%Y-%m-%d")
            .expect("Invalid date format, expected YYYY-MM-DD"),
        None => now.date_naive(),
    };

    let time = match &cli.at {
        Some(t) => NaiveTime::parse_from_str(t, "%H:%M")
            .expect("Invalid time format, expected HH:MM"),
        None => now.time(),
    };

    let naive_dt = date.and_time(time);
    let utc_dt = Utc.from_utc_datetime(&naive_dt);

    let dt_type = if cli.arrival {
        SearchDateTimeType::Arrival
    } else {
        SearchDateTimeType::Departure
    };

    debug!(dbg, "endpoint: {}", sbb_api::API_ENDPOINT);
    debug!(dbg, "from: {:?} (ref: {:?})", cli.from, cli.from_ref);
    debug!(dbg, "to:   {:?} (ref: {:?})", cli.to, cli.to_ref);
    debug!(dbg, "date: {}, time: {}, type: {}", date, time, dt_type);
    debug!(dbg, "connecting...");

    let request = get_connections(
        &cli.from,
        cli.from_ref.as_deref(),
        &cli.to,
        cli.to_ref.as_deref(),
        &utc_dt,
        dt_type,
    );

    let resp = match timeout(Duration::from_secs(15), request).await {
        Ok(Ok(r)) => {
            debug!(dbg, "got response, {} trip(s)", r.trips.len());
            r
        }
        Ok(Err(e)) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
        Err(_) => {
            eprintln!("error: request timed out after 15 seconds");
            std::process::exit(1);
        }
    };

    for (i, trip) in resp.trips.iter().enumerate() {
        let summary = &trip.summary;

        let transport = summary
            .departure_anchor
            .transport_designation
            .as_ref()
            .map(|t| t.to_string())
            .unwrap_or_else(|| "?".to_string());

        let direction = summary
            .departure_anchor
            .direction
            .as_deref()
            .map(|d| format!(" → {}", d))
            .unwrap_or_default();

        let duration_str = summary
            .duration
            .as_ref()
            .map(|d| d.localized_label.as_str())
            .unwrap_or("?");

        let transfers = trip.transfers();
        let transfer_text = match transfers {
            0 => "direct".to_string(),
            1 => "1 transfer".to_string(),
            n => format!("{} transfers", n),
        };

        println!(
            "{}. {}{} ({}, {})",
            i + 1,
            transport,
            direction,
            duration_str,
            transfer_text,
        );

        // Print per-leg stop info if detail is available
        if let Some(detail) = &trip.detail {
            for leg in &detail.legs {
                if let TripLeg::PtRideLeg(pt) = leg {
                    let dep = &pt.departure_stop_point;
                    let arr = &pt.arrival_stop_point;

                    let dep_time = dep
                        .departure_time
                        .as_ref()
                        .and_then(|t| t.display_time.as_deref())
                        .unwrap_or("?");
                    let arr_time = arr
                        .arrival_time
                        .as_ref()
                        .and_then(|t| t.display_time.as_deref())
                        .unwrap_or("?");

                    let track = dep
                        .quay
                        .as_ref()
                        .filter(|q| !q.name.is_empty())
                        .map(|q| {
                            if q.changed {
                                format!("   [Pl. {}!]", q.name)
                            } else {
                                format!("   [Pl. {}]", q.name)
                            }
                        })
                        .unwrap_or_default();

                    println!(
                        "   {}  {}  →  {}  {}{}",
                        dep.display_name, dep_time, arr.display_name, arr_time, track,
                    );
                }
            }
        } else {
            // Fall back to summary anchors when no detail is available
            let dep = &summary.departure_anchor;
            let arr = &summary.arrival_anchor;
            let track = dep
                .quay
                .as_ref()
                .filter(|q| !q.name.is_empty())
                .map(|q| format!("   [Pl. {}]", q.name))
                .unwrap_or_default();
            println!(
                "   {}  {}  →  {}  {}{}",
                dep.place_name, dep.display_time, arr.place_name, arr.display_time, track,
            );
        }

        println!();
    }

    if resp.trips.is_empty() {
        println!("No connections found.");
    }
}
