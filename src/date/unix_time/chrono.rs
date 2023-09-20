use chrono::NaiveDate;

pub(crate) struct ChronoCalc {
    pub(crate) year: i32,
   pub(crate) month: Option<u32>,
    pub(super) day: Option<u32>,
}

impl ChronoCalc {
    #[inline(always)]
    pub(super) fn _calc(&self) -> Result<u64, String> {
        let date = NaiveDate::from_ymd_opt(self.year, self.month.unwrap_or(1), self.day.unwrap_or(1))
            .ok_or_else(|| "Invalid date".to_string())?;
        Ok(date.and_hms_opt(0, 0, 0).expect("Internal error").timestamp() as u64)
    }
}
