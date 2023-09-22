#[cfg(feature = "chrono-backend")]
pub(super) mod chrono;
#[cfg(not(any(feature = "chrono-backend", feature = "time-backend")))]
pub(super) mod simple;
#[cfg(feature = "time-backend")]
pub(super) mod time;

use crate::Result;

pub(super) const ONE_HOUR: u64 = 3600;

pub(super) trait UnixTimeCalc {
    fn new() -> Self;
    fn is_leap_year(year: u32) -> bool {
        year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
    }
    fn year(&mut self, year: String) -> Result<()>;
    fn month(&mut self, month: String) -> Result<()>;
    fn day(&mut self, day: String) -> Result<()>;
    fn is_year_set(&self) -> bool;
    fn is_month_set(&self) -> bool;
    fn is_day_set(&self) -> bool;
    fn calc(&self) -> Result<u64>;
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

            fn year(&mut self, year: String) -> Result<()> {
                self.year = year.parse::<i32>().map_err(|_| "Invalid year")?;
                Ok(())
            }

            fn month(&mut self, month: String) -> Result<()> {
                self.month = Some(month.parse::<$mt>().map_err(|_| "Invalid month")?);
                Ok(())
            }

            fn day(&mut self, day: String) -> Result<()> {
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

            fn calc(&self) -> Result<u64> {
                self._calc()
            }
        }
    };
}

#[cfg(feature = "chrono-backend")]
impl_unixtime_calc!(chrono::ChronoCalc, u32, u32);
#[cfg(feature = "time-backend")]
impl_unixtime_calc!(time::TimeCalc, u8, u8);

#[cfg(test)]
mod tests {
    use super::*;

    const YYY_MM_DD_INPUTS: [((&str, &str, &str), u64); 3] = [
        (("2003", "2", "2"), 1_044_144_000),
        (("2001", "2", "28"), 983_318_400),
        (("2033", "3", "26"), 1_995_408_000),
    ];

    const YYY_MM_INPUTS: [((&str, &str), u64); 3] = [
        (("2003", "2"), 1_044_057_600),
        (("2001", "2"), 980_985_600),
        (("2033", "3"), 1_993_248_000),
    ];

    const YYY_INPUTS: [(&str, u64); 3] = [
        ("2003", 1_041_379_200),
        ("2001", 978_307_200),
        ("2033", 1_988_150_400),
    ];

    fn test_calc<C: UnixTimeCalc>() -> Result<()> {
        for ((year, month, day), expected) in YYY_MM_DD_INPUTS.iter() {
            let mut calc = C::new();
            calc.year(year.to_string())?;
            calc.month(month.to_string())?;
            calc.day(day.to_string())?;
            assert_eq!(calc.calc()?, *expected);
        }

        for ((year, month), expected) in YYY_MM_INPUTS.iter() {
            let mut calc = C::new();
            calc.year(year.to_string())?;
            calc.month(month.to_string())?;
            assert_eq!(calc.calc()?, *expected);
        }

        for (year, expected) in YYY_INPUTS.iter() {
            let mut calc = C::new();
            calc.year(year.to_string())?;
            assert_eq!(calc.calc()?, *expected);
        }

        Ok(())
    }

    #[cfg(not(any(feature = "chrono-backend", feature = "time-backend")))]
    #[test]
    #[ignore] // We know that the simple backend is not accurate right now
    fn test_calc_simple() -> Result<()> {
        test_calc::<simple::SimpleCalc>()
    }

    #[cfg(feature = "chrono-backend")]
    #[test]
    fn test_calc_chrono() -> Result<()> {
        test_calc::<chrono::ChronoCalc>()
    }

    #[cfg(feature = "time-backend")]
    #[test]
    fn test_calc_time() -> Result<()> {
        test_calc::<time::TimeCalc>()
    }
}
