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
