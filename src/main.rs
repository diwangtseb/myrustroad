mod hundred_case;

use chrono::{TimeZone, Utc};
use hundred_case::{one_day::hello_world};
use hundred_case::two_day::time_after;
fn main() {
    hello_world();
    let res = time_after(Utc.ymd(2015, 1, 24).and_hms(23, 59, 59));
    println!("{}",res)
}