extern crate chrono;
use chrono::{DateTime, Duration, Utc};

pub fn time_after(start: DateTime<Utc>) -> DateTime<Utc> {
    start + Duration::seconds(0b111011100110101100101000000000)
 }