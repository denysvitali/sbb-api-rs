use clap::{CommandFactory, Parser};
use clap_complete::Shell;
use colored::control;
use colored::Colorize;
use chrono::{DateTime, Local, NaiveDate, NaiveTime, TimeZone, Utc};
use reqwest::StatusCode;
use serde::Deserialize;
use sbb_api::connections::get_connections;
use sbb_api::models::location::SearchDateTimeType;
use sbb_api::models::trip::TripLeg;
use std::time::{Duration, Instant};
use tokio::time::timeout;

// Exit codes: 0=success, 1=error, 2=no results
const EXIT_SUCCESS: i32 = 0;
const EXIT_ERROR: i32 = 1;
const EXIT_NO_RESULTS: i32 = 2;

// Timeout duration for requests
const REQUEST_TIMEOUT_SECS: u64 = 15;

/// SBB API error response structure (for parsing 400 Bad Request errors)
#[derive(Deserialize, Debug)]
struct SbbApiError {
    #[serde(default)]
    message: Option<String>,
    #[serde(default)]
    error: Option<String>,
    #[serde(default)]
    error_description: Option<String>,
}

/// Error types for better handling
enum AppError {
    Timeout,
    HttpError(StatusCode, String),
    NetworkError(String),
    ParseError(String),
}

impl AppError {}

/// Print error message in red with optional retry suggestion
fn print_error(msg: &str, show_retry: bool) {
    eprintln!("{} {}", "error:".red().bold(), msg.red());
    if show_retry {
        eprintln!("{} {}", "hint:".yellow().bold(), "Please check your network connection and try again.".yellow());
    }
}

/// Print error message in red (no retry suggestion)
fn print_error_simple(msg: &str) {
    print_error(msg, false);
}

/// Try to parse API error message from HTTP response body
fn parse_api_error(body: &str, status: StatusCode) -> String {
    // Try to parse as SBB API error JSON
    if let Ok(api_error) = serde_json::from_str::<SbbApiError>(body) {
        if let Some(msg) = api_error.message {
            return msg;
        }
        if let Some(msg) = api_error.error {
            return msg;
        }
        if let Some(msg) = api_error.error_description {
            return msg;
        }
    }

    // If JSON parsing fails but body is not empty, return a truncated version of body
    if !body.is_empty() {
        let truncated = if body.len() > 200 {
            format!("{}...", &body[..200])
        } else {
            body.to_string()
        };
        return format!("Server returned: {}", truncated);
    }

    // Fallback to status description
    status.canonical_reason().unwrap_or("Unknown error").to_string()
}

/// Convert SimpleError to AppError with better context
fn convert_error(err: simple_error::SimpleError) -> AppError {
    let err_str = err.to_string();

    // Check for common network error patterns
    let lower_err = err_str.to_lowercase();
    if lower_err.contains("connection")
        || lower_err.contains("network")
        || lower_err.contains("dns")
        || lower_err.contains("socket")
        || lower_err.contains("tcp")
    {
        return AppError::NetworkError(err_str);
    }

    // Check for HTTP errors (format: "HTTP {status}: {body}")
    if let Some(pos) = err_str.find("HTTP ") {
        let http_part = &err_str[pos..];
        if let Some(space_pos) = http_part.find(' ') {
            let status_str = &http_part[space_pos + 1..];
            if let Some(colon_pos) = status_str.find(':') {
                let status_code_str = &status_str[..colon_pos];
                let body = status_str[colon_pos + 1..].trim();

                if let Ok(status) = status_code_str.parse::<StatusCode>() {
                    return AppError::HttpError(status, body.to_string());
                }
            }
        }
    }

    // Check for timeout in error message
    if lower_err.contains("timeout") {
        return AppError::Timeout;
    }

    // Check for JSON parse errors
    if lower_err.contains("json") || lower_err.contains("parse") {
        return AppError::ParseError(err_str);
    }

    // Default: treat as unknown HTTP error
    AppError::ParseError(err_str)
}

/// Handle HTTP errors with detailed messages and helpful hints
fn handle_http_error(status: StatusCode, body: &str) {
    let error_msg = if status == StatusCode::BAD_REQUEST {
        // Special handling for 400 Bad Request - try to show API error message
        parse_api_error(body, status)
    } else {
        // For other HTTP errors, show status and truncated body
        if body.is_empty() {
            status.canonical_reason().unwrap_or("HTTP error").to_string()
        } else {
            let truncated = if body.len() > 200 {
                format!("{}...", &body[..200])
            } else {
                body.to_string()
            };
            format!(
                "HTTP {} {} - {}",
                status.as_u16(),
                status.canonical_reason().unwrap_or(""),
                truncated
            )
        }
    };

    // Show HTTP errors in red
    print_error_simple(&error_msg);

    // Add specific hint for certain status codes
    if status == StatusCode::UNAUTHORIZED || status == StatusCode::FORBIDDEN {
        eprintln!("{} {}", "hint:".yellow().bold(), "The API may have changed. Consider updating the application.".yellow());
    } else if status == StatusCode::TOO_MANY_REQUESTS {
        eprintln!("{} {}", "hint:".yellow().bold(), "You are being rate-limited. Please wait a moment and try again.".yellow());
    } else if status == StatusCode::BAD_REQUEST {
        eprintln!("{} {}", "hint:".yellow().bold(), "Check your station names or UIC references for typos.".yellow());
    }
}

// Enable colors based on terminal support and --no-color flag
fn init_colors(no_color: bool) {
    if no_color {
        control::set_override(false);
    } else if atty::is(atty::Stream::Stdout) {
        control::set_override(true);
    } else {
        control::set_override(false);
    }
}

#[derive(Parser)]
#[command(name = "sbb", about = "Query SBB train connections", version)]
struct Cli {
    /// Departure station or address
    #[arg(group = "from_group", value_name = "FROM")]
    from: Option<String>,
    /// Arrival station or address
    #[arg(group = "to_group", value_name = "TO")]
    to: Option<String>,
    /// UIC reference for departure (e.g. 8503000 for Zürich HB)
    #[arg(long = "from-ref", value_name = "UIC", group = "from_group")]
    from_ref: Option<String>,
    /// UIC reference for arrival
    #[arg(long = "to-ref", value_name = "UIC", group = "to_group")]
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
    /// Output raw JSON response
    #[arg(short = 'j', long = "json")]
    json: bool,
    /// Disable colored output
    #[arg(long = "no-color")]
    no_color: bool,
    /// Generate shell completions
    #[arg(long = "generate", value_name = "SHELL", value_enum)]
    generate: Option<Shell>,
}

macro_rules! debug {
    ($enabled:expr, $($arg:tt)*) => {
        if $enabled {
            eprintln!("[debug] {}", format!($($arg)*));
        }
    };
}

/// Calculate delay in minutes between aimed and expected time
fn calculate_delay_minutes(time_aimed: &Option<String>, time_expected: &Option<String>) -> Option<i64> {
    let aimed = time_aimed.as_ref()?;
    let expected = time_expected.as_ref()?;

    // Parse ISO 8601 datetime strings (e.g., "2025-02-22T10:32:00+01:00")
    let aimed_dt = DateTime::parse_from_rfc3339(aimed).ok()?;
    let expected_dt = DateTime::parse_from_rfc3339(expected).ok()?;

    let diff = expected_dt.signed_duration_since(aimed_dt);
    let diff_minutes = diff.num_minutes();

    if diff_minutes > 0 {
        Some(diff_minutes)
    } else {
        None
    }
}

/// Format delay as a string (e.g., "+5 min" or empty string if on time)
fn format_delay(time_aimed: &Option<String>, time_expected: &Option<String>) -> String {
    if let Some(delay) = calculate_delay_minutes(time_aimed, time_expected) {
        if delay > 0 {
            return format!(" +{} min", delay);
        }
    }
    String::new()
}

/// Parse datetime from ISO string to get just the time portion
fn parse_iso_time(iso_str: &str) -> Option<NaiveTime> {
    DateTime::parse_from_rfc3339(iso_str)
        .ok()
        .map(|dt| dt.time())
}

/// Format time with optional delay indicator
fn format_time_with_delay(
    time_aimed: &Option<String>,
    time_expected: &Option<String>,
) -> String {
    let delay_str = format_delay(time_aimed, time_expected);

    if let Some(aimed) = time_aimed {
        if let Some(time) = parse_iso_time(aimed) {
            return format!("{}{}", time.format("%H:%M"), delay_str);
        }
    }

    // Fallback to display_time if available (but this doesn't have delay info)
    "?".to_string()
}

#[actix_rt::main]
async fn main() {
    let cli = Cli::parse();

    // Initialize colors based on terminal support and --no-color flag
    init_colors(cli.no_color);

    // Handle shell completion generation
    if let Some(shell) = cli.generate {
        let mut cmd = Cli::command();
        clap_complete::generate(shell, &mut cmd, "sbb", &mut std::io::stdout());
        std::process::exit(EXIT_SUCCESS);
    }

    // Validate required arguments: either text or UIC ref must be provided for both from and to
    let from_input = cli.from.is_some() || cli.from_ref.is_some();
    let to_input = cli.to.is_some() || cli.to_ref.is_some();

    if !from_input {
        print_error_simple("either FROM or --from-ref must be provided");
        std::process::exit(EXIT_ERROR);
    }
    if !to_input {
        print_error_simple("either TO or --to-ref must be provided");
        std::process::exit(EXIT_ERROR);
    }

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

    // Print SBB header
    println!("{}", "SBB Connections".bold().cyan());
    println!();

    // Print search parameters
    print!("{} ", "From:".bold().white());
    println!("{}", cli.from.clone().unwrap_or_default());
    print!("{} ", "To:".bold().white());
    println!("{}", cli.to.clone().unwrap_or_default());
    if let Some(ref from_ref) = cli.from_ref {
        println!("{} {}", "From Ref:".bold().white(), from_ref);
    }
    if let Some(ref to_ref) = cli.to_ref {
        println!("{} {}", "To Ref:".bold().white(), to_ref);
    }
    println!("{} {} ({})", "Date:".bold().white(), date.format("%Y-%m-%d (%a)"), time.format("%H:%M"));
    println!();

    // Enhanced debug output with clear sections
    debug!(dbg, "----------------------------------------");
    debug!(dbg, "API endpoint: {}", sbb_api::API_ENDPOINT);
    debug!(dbg, "----------------------------------------");
    debug!(dbg, "Search parameters (detailed):");
    debug!(dbg, "  from:       {:?}", cli.from);
    debug!(dbg, "  from_ref:   {:?}", cli.from_ref);
    debug!(dbg, "  to:         {:?}", cli.to);
    debug!(dbg, "  to_ref:     {:?}", cli.to_ref);
    debug!(dbg, "  date:       {}", date);
    debug!(dbg, "  time:       {}", time);
    debug!(dbg, "  datetime_type: {}", dt_type);
    debug!(dbg, "  utc_dt:     {}", utc_dt);
    debug!(dbg, "----------------------------------------");
    debug!(dbg, "Connecting to API...");

    let start_time = Instant::now();

    let request = get_connections(
        cli.from.as_deref().unwrap_or(""),
        cli.from_ref.as_deref(),
        cli.to.as_deref().unwrap_or(""),
        cli.to_ref.as_deref(),
        &utc_dt,
        dt_type,
    );

    let resp = match timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS), request).await {
        Ok(Ok(r)) => {
            let elapsed = start_time.elapsed();
            debug!(dbg, "Response received in {:.2?}", elapsed);
            debug!(dbg, "Found {} trip(s)", r.trips.len());
            r
        }
        Ok(Err(e)) => {
            // Convert the error to our AppError type for better handling
            let app_error = convert_error(e);

            match app_error {
                AppError::Timeout => {
                    print_error(
                        &format!("request timed out after {} seconds", REQUEST_TIMEOUT_SECS),
                        true,
                    );
                    std::process::exit(EXIT_ERROR);
                }
                AppError::NetworkError(msg) => {
                    // Show connection errors in red with retry suggestion
                    print_error(&format!("connection failed: {}", msg), true);
                    std::process::exit(EXIT_ERROR);
                }
                AppError::HttpError(status, body) => {
                    // Handle HTTP errors with detailed messages
                    handle_http_error(status, &body);
                    std::process::exit(EXIT_ERROR);
                }
                AppError::ParseError(msg) => {
                    // Show parse errors in red
                    print_error(&format!("failed to parse response: {}", msg), false);
                    std::process::exit(EXIT_ERROR);
                }
            }
        }
        Err(_) => {
            // Timeout case - handle gracefully
            print_error(
                &format!("request timed out after {} seconds", REQUEST_TIMEOUT_SECS),
                true,
            );
            std::process::exit(EXIT_ERROR);
        }
    };

    let elapsed = start_time.elapsed();

    // Output JSON if requested
    if cli.json {
        let json_output = serde_json::to_string_pretty(&resp).expect("Failed to serialize JSON");
        println!("{}", json_output);
        std::process::exit(EXIT_SUCCESS);
    }

    // Print results
    if resp.trips.is_empty() {
        // This is NOT an error - just no connections found for the given criteria
        println!("{}", "No connections found for the specified route and time.".yellow());
        debug!(dbg, "no trips returned from API");
        std::process::exit(EXIT_NO_RESULTS);
    }

    // Results header
    println!("{}", "Connections".bold().underline());
    println!();

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
            .map(|d| format!(" -> {}", d))
            .unwrap_or_default();

        let duration_str = summary
            .duration
            .as_ref()
            .map(|d| d.localized_label.as_str())
            .unwrap_or("?");

        let transfers = trip.transfers();
        let _transfer_text = match transfers {
            0 => "direct".to_string(),
            1 => "1 transfer".to_string(),
            n => format!("{} transfers", n),
        };

        // Get departure time with delay
        let dep_time = format_time_with_delay(
            &summary.departure_anchor.time_aimed,
            &summary.departure_anchor.time_expected,
        );

        // Get arrival time with delay
        let arr_time = format_time_with_delay(
            &summary.arrival_anchor.time_aimed,
            &summary.arrival_anchor.time_expected,
        );

        // Print trip summary line
        println!(
            "{}. {} → {}  {}  {}{}",
            (i + 1).to_string().bold().white(),
            dep_time,
            arr_time,
            duration_str.yellow(),
            transport.bold().cyan(),
            direction,
        );

        // Print per-leg stop info if detail is available
        if let Some(detail) = &trip.detail {
            for leg in &detail.legs {
                if let TripLeg::PtRideLeg(pt) = leg {
                    let dep = &pt.departure_stop_point;
                    let arr = &pt.arrival_stop_point;

                    // Departure time with delay
                    let dep_time_leg = format_time_with_delay(
                        &dep.departure_time.as_ref().and_then(|t| t.time_aimed.clone()),
                        &dep.departure_time.as_ref().and_then(|t| t.time_expected.clone()),
                    );

                    // Arrival time with delay
                    let _arr_time_leg = format_time_with_delay(
                        &arr.arrival_time.as_ref().and_then(|t| t.time_aimed.clone()),
                        &arr.arrival_time.as_ref().and_then(|t| t.time_expected.clone()),
                    );

                    let track = dep
                        .quay
                        .as_ref()
                        .filter(|q| !q.name.is_empty())
                        .map(|q| {
                            if q.changed {
                                format!(" [Platform {}!]", q.name).bold().red()
                            } else {
                                format!(" [Platform {}]", q.name).bold().yellow()
                            }
                        })
                        .unwrap_or_default();

                    // Get transport info for this leg
                    let leg_transport = pt
                        .first_transport_designation
                        .as_ref()
                        .map(|t| t.to_string())
                        .unwrap_or_else(|| "".to_string());

                    println!(
                        "      {}  {} → {}  {}{}",
                        leg_transport.bold().cyan(),
                        dep.display_name.white(),
                        arr.display_name.white(),
                        dep_time_leg.bold().green(),
                        track,
                    );
                } else if let TripLeg::AccessLeg(access) = leg {
                    // Walking/transfer legs
                    let dep_name = access
                        .departure_stop_point
                        .as_ref()
                        .map(|s| s.display_name.as_str())
                        .unwrap_or("");
                    let arr_name = access
                        .arrival_stop_point
                        .as_ref()
                        .map(|s| s.display_name.as_str())
                        .unwrap_or("");
                    if !dep_name.is_empty() && !arr_name.is_empty() {
                        println!(
                            "      {}  {} → {}",
                            "Walk".italic().dimmed(),
                            dep_name.white().dimmed(),
                            arr_name.white().dimmed(),
                        );
                    }
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
                .map(|q| format!("  [Pl. {}]", q.name).bold().yellow())
                .unwrap_or_default();
            println!(
                "      {} → {}  {}{}",
                dep.place_name.white(),
                arr.place_name.white(),
                dep.display_time.bold().green(),
                track,
            );
        }

        println!();
    }

    // Print summary line
    println!(
        "{} {}",
        "Total:".bold().white(),
        format!("{} connection(s) found in {:.2?}", resp.trips.len(), elapsed).dimmed()
    );

    // Success - exit with code 0
    std::process::exit(EXIT_SUCCESS);
}
