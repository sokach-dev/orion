use chrono::{DateTime, NaiveDateTime, Utc};
use prost_types::Timestamp;

pub fn convert_to_utc_time(ts: &Timestamp) -> DateTime<Utc> {
    DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp_micros(ts.seconds * 1000000).unwrap(),
        Utc,
    )
}

pub fn convert_to_timestamp(dt: &DateTime<Utc>) -> Timestamp {
    Timestamp {
        seconds: dt.timestamp(),
        nanos: dt.timestamp_subsec_nanos() as _,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn convert_to_utc_time_should_work() {
        let dt = Utc.with_ymd_and_hms(2023, 03, 03, 00, 00, 00).unwrap();

        let ts = Timestamp {
            seconds: 1677801600,
            nanos: 0,
        };
        assert_eq!(convert_to_utc_time(&ts), dt);
    }

    #[test]
    fn convert_to_timestamp_should_work() {
        let dt = Utc.with_ymd_and_hms(2023, 03, 03, 00, 00, 00).unwrap();

        let ts = Timestamp {
            seconds: 1677801600,
            nanos: 0,
        };
        assert_eq!(convert_to_timestamp(&dt), ts);
    }
}
