use core::fmt;
use std::fmt::Formatter;

pub enum LocationType {
    Address,
    Station
}

impl fmt::Display for LocationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            LocationType::Address=> {
                write!(f, "a")
            },
            LocationType::Station => {
                write!(f, "s")
            }
        }
    }
}