use crate::date::unix_time::UnixTimeCalc;
use chrono::NaiveDate;

pub(crate) struct ChronoCalc;

impl UnixTimeCalc for ChronoCalc {
    fn calc(year: u32, month: u32, day: u32) -> i64 {
        let date =
            NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32).expect("Invalid date");

        date.and_hms_opt(0, 0, 0).expect("Invalid time").timestamp() as i64
    }
}
