use crate::date::unix_time::UnixTimeCalc;
use chrono::NaiveDate;

pub(crate) struct ChronoCalc {
    year: i32,
    month: Option<u32>,
    day: Option<u32>,
}

impl UnixTimeCalc for ChronoCalc {
    fn new() -> Self {
        Self {
            year: 0,
            month: None,
            day: None,
        }
    }

    fn year(&mut self, year: String) -> Result<(), String> {
        self.year = year.parse::<i32>().map_err(|_| "Invalid year")?;
        Ok(())
    }

    fn month(&mut self, month: String) -> Result<(), String> {
        self.month = Some(month.parse::<u32>().map_err(|_| "Invalid month")?);
        Ok(())
    }

    fn day(&mut self, day: String) -> Result<(), String> {
        self.day = Some(day.parse::<u32>().map_err(|_| "Invalid day")?);
        Ok(())
    }

    fn is_year_set(&self) -> bool {
        self.year != 0
    }

    fn is_month_set(&self) -> bool {
        self.month.is_some()
    }

    fn is_day_set(&self) -> bool {
        self.day.is_some()
    }


    fn calc(&self) -> Result<u64, String> {
        let date = NaiveDate::from_ymd_opt(self.year, self.month.unwrap_or(1), self.day.unwrap_or(1))
            .ok_or_else(|| "Invalid date".to_string())?;
        Ok(date.and_hms_opt(0, 0, 0).expect("Internal error").timestamp() as u64)
    }
}
