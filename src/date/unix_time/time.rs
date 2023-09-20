use crate::Result;
use time::{Date, Month, PrimitiveDateTime, Time};

pub(crate) struct TimeCalc {
    pub(super) year: i32,
    pub(super) month: Option<u8>,
    pub(super) day: Option<u8>,
}

impl TimeCalc {
    #[inline(always)]
    pub(super) fn _calc(&self) -> Result<u64> {
        let Ok(dt) = Date::from_calendar_date(
            self.year,
            Month::try_from(self.month.unwrap_or(1)).unwrap(),
            self.day.unwrap_or(1),
        ) else {
            return Err("Invalid date".to_string());
        };
        let dt = PrimitiveDateTime::new(dt, Time::from_hms(0, 0, 0).expect("Internal error"));
        Ok(dt.assume_utc().unix_timestamp() as u64)
    }
}
