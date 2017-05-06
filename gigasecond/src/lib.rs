extern crate chrono;
use chrono::*;

pub fn after(start_date : DateTime<UTC>) -> DateTime<UTC> {
    start_date + Duration::seconds(1_000_000_000)
}
