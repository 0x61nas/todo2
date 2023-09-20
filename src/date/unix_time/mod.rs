#[cfg(feature = "chrono-backend")]
pub(super) mod chrono;
#[cfg(not(any(feature = "chrono-backend", feature = "time-backend")))]
pub(super) mod simple;
#[cfg(feature = "time-backend")]
pub(super) mod time;

pub(super) const ONE_HOUR: u64 = 3600;

pub(super) trait UnixTimeCalc {
    fn new() -> Self;
    fn is_leap_year(year: u32) -> bool {
        year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
    }
    fn year(&mut self, year: String) -> Result<(), String>;
    fn month(&mut self, month: String) -> Result<(), String>;
    fn day(&mut self, day: String) -> Result<(), String>;
    fn is_year_set(&self) -> bool;
    fn is_month_set(&self) -> bool;
    fn is_day_set(&self) -> bool;
    fn calc(&self) -> Result<u64, String>;
}

macro_rules! impl_unixtime_calc {
    ($t:ty, $mt:ty, $dt:ty) => {
            impl UnixTimeCalc for $t {
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
                    self.month = Some(month.parse::<$mt>().map_err(|_| "Invalid month")?);
                    Ok(())
                }

                fn day(&mut self, day: String) -> Result<(), String> {
                    self.day = Some(day.parse::<$dt>().map_err(|_| "Invalid day")?);
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
                    self._calc()
                }
            }
    };
}

#[cfg(feature = "chrono-backend")]
impl_unixtime_calc!(chrono::ChronoCalc, u32, u32);
#[cfg(feature = "time-backend")]
impl_unixtime_calc!(time::TimeCalc, u8, u8);
