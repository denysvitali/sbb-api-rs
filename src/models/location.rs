use core::fmt;

/// Search date/time interpretation for connection queries.
#[derive(Debug, Clone, Copy)]
pub enum SearchDateTimeType {
    Departure,
    Arrival,
}

impl fmt::Display for SearchDateTimeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SearchDateTimeType::Departure => write!(f, "DEPARTURE"),
            SearchDateTimeType::Arrival => write!(f, "ARRIVAL"),
        }
    }
}
