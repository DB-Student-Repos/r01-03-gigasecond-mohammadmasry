use time::PrimitiveDateTime as DateTime;
use time::Duration;

pub fn after(start: DateTime) -> DateTime {
    let gigasecond_duration = Duration::seconds(1_000_000_000);
    start + gigasecond_duration
}
