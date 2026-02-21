# sbb-api-rs

<p align="center">
<img src="./resources/logo.png" height="60"/>
</p>

A Rust library and CLI tool for querying SBB (Swiss Federal Railways) connections using the v2 Timetable API.

## Overview

This project provides a Rust library and command-line interface for accessing SBB's unofficial v2 Timetable API (`https://active.vnext.app.sbb.ch/api/timetable/v2/trips`). It allows you to search for train, bus, tram, and other public transport connections between any two locations in Switzerland.

### Features

- Asynchronous API calls using `reqwest` and `tokio`
- Custom authentication using HMAC-SHA1 signing
- Custom CA certificate handling for SBB's self-signed certificate
- Support for both departure and arrival time searches
- UIC station reference support for precise station matching
- Detailed trip information including legs, platforms, and transfers

## Installation

### Prerequisites

- Rust 1.70 or later
- Cargo

### As a CLI Tool

```bash
# Clone the repository
git clone https://github.com/denysvitali/sbb-api-rs.git
cd sbb-api-rs

# Build and install the CLI
cargo install --path .
```

### As a Library

Add this to your `Cargo.toml`:

```toml
[dependencies]
sbb-api = { path = "/path/to/sbb-api-rs" }
```

Or from the GitHub repository:

```toml
[dependencies]
sbb-api = { git = "https://github.com/denysvitali/sbb-api-rs.git" }
```

## Usage

### CLI Examples

#### Basic connection search

```bash
sbb "Zürich HB" "Basel SBB"
```

#### Search for a specific date and time

```bash
sbb "Zürich HB" "Bern" --date 2026-03-15 --at 14:30
```

#### Search by arrival time instead of departure time

```bash
sbb "Zürich HB" "Basel SBB" --arrival --at 18:00
```

#### Using UIC station references for precise matching

```bash
sbb "Zürich HB" "Basel SBB" --from-ref 8503000 --to-ref 8500010
```

#### Show debug information

```bash
sbb "Zürich HB" "Luzern" -d
```

### CLI Options

| Option | Description |
|--------|-------------|
| `from` | Departure station or address (required) |
| `to` | Arrival station or address (required) |
| `--from-ref`, `--from-ref <UIC>` | UIC station reference for departure (e.g., `8503000` for Zürich HB) |
| `--to-ref`, `--to-ref <UIC>` | UIC station reference for arrival |
| `--at`, `--at <HH:MM>` | Departure/arrival time (default: current time) |
| `--date`, `--date <YYYY-MM-DD>` | Departure/arrival date (default: today) |
| `--arrival` | Search for connections arriving at the specified time instead of departing |
| `-d`, `--debug` | Print debug information to stderr |

### Library Examples

#### Basic connection search

```rust
use sbb_api::connections::get_connections;
use sbb_api::models::location::SearchDateTimeType;
use chrono::Utc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = Utc::now();

    let result = get_connections(
        "Zürich HB",
        Some("8503000"),  // UIC code for Zürich HB
        "Basel SBB",
        Some("8500010"),  // UIC code for Basel SBB
        &now,
        SearchDateTimeType::Departure,
    ).await?;

    println!("Found {} connections:", result.trips.len());

    for trip in &result.trips {
        let summary = &trip.summary;
        println!(
            "{} → {} ({})",
            summary.departure_anchor.display_time,
            summary.arrival_anchor.display_time,
            summary.duration.as_ref()
                .map(|d| d.localized_label.as_str())
                .unwrap_or("?")
        );
    }

    Ok(())
}
```

#### Search with arrival time

```rust
use sbb_api::connections::get_connections;
use sbb_api::models::location::SearchDateTimeType;
use chrono::{Utc, TimeZone};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set arrival time to 6 PM today
    let today = chrono::offset::Local::now();
    let arrival = Utc.with_ymd_and_hms(
        today.year(),
        today.month(),
        today.day(),
        18, 0, 0
    ).unwrap();

    let result = get_connections(
        "Zürich HB",
        None,  // Let the API resolve the station
        "Bern",
        None,
        &arrival,
        SearchDateTimeType::Arrival,  // Search by arrival time
    ).await?;

    Ok(())
}
```

## API Parameters

The `get_connections` function supports the following parameters:

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `from` | `&str` | Yes | Departure station name or address |
| `from_ref` | `Option<&str>` | No | UIC station reference for departure (e.g., `8503000` for Zürich HB) |
| `to` | `&str` | Yes | Arrival station name or address |
| `to_ref` | `Option<&str>` | No | UIC station reference for arrival |
| `on` | `&DateTime<Utc>` | Yes | Date and time for the search |
| `dt_type` | `SearchDateTimeType` | Yes | Either `Departure` or `Arrival` |

### UIC Station References

UIC station references are unique identifiers for stations. Some common references:

| Station | UIC Code |
|---------|----------|
| Zürich HB | 8503000 |
| Basel SBB | 8500010 |
| Bern | 8507000 |
| Luzern | 8505000 |
| Genève | 8501001 |
| Lausanne | 8502000 |

## Building and Testing

### Build

```bash
# Build the library
cargo build

# Build the CLI binary
cargo build --release
```

### Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture
```

Note: Some tests are marked as `#[ignore]` because they require live API access. To run them:

```bash
cargo test -- --ignored
```

### Running the CLI from source

```bash
# Run with cargo
cargo run -- "Zürich HB" "Basel SBB"

# With options
cargo run -- "Zürich HB" "Bern" --date 2026-03-15 --at 14:30
```

## Technical Details

### API Endpoint

- **URL**: `https://active.vnext.app.sbb.ch/api/timetable/v2/trips`
- **Protocol**: HTTPS with custom CA certificate

### Authentication

The library implements custom authentication using:

1. **App Token**: A random UUID for each request
2. **API Authorization**: HMAC-SHA1 signature of the path + date

### CA Certificate

The library includes SBB's self-signed root CA certificate (`*.sbbmobile.ch`) for certificate verification.

### Dependencies

- `reqwest` - HTTP client
- `tokio` - Async runtime
- `chrono` - Date/time handling
- `serde` / `serde_json` - Serialization
- `clap` - CLI argument parsing
- `openssl` - Cryptographic operations
- `uuid` - Token generation

## License

MIT License - See [LICENSE](LICENSE) for details.

## Disclaimer

This is an unofficial API wrapper. It is not affiliated with or endorsed by SBB (Swiss Federal Railways). Use at your own risk. The API may change without notice.
