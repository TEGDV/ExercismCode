use std::ops::Add;

use chrono::{DateTime, Duration, Utc};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    println!("the start var is {}", start);
    start + Duration::seconds(1_000_000_000)
}
