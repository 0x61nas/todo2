use crate::date::unix_time::UnixTimeCalc;
use crate::Result;

const EIGHT_HOURS: u64 = 28_798;
const ONE_DAY: u64 = 86_400;
const ONE_MONTH: u64 = 2_629_743;
const ONE_YEAR: u64 = 31_556_926;

pub(crate) struct SimpleCalc {
    counter: u8,
    time_stamp: u64,
}

impl UnixTimeCalc for SimpleCalc {
    fn new() -> Self {
        Self {
            counter: 0,
            time_stamp: 0,
        }
    }

    fn year(&mut self, year: String) -> Result<()> {
        self.time_stamp += (year.parse::<u64>().map_err(|_| "Invalid year")? - 1970) * ONE_YEAR;
        self.counter += 1;
        Ok(())
    }

    fn month(&mut self, month: String) -> Result<()> {
        self.time_stamp += month.parse::<u64>().map_err(|_| "Invalid month")? * ONE_MONTH;
        self.counter += 1;
        Ok(())
    }

    fn day(&mut self, day: String) -> Result<()> {
        self.time_stamp += day.parse::<u64>().map_err(|_| "Invalid day")? * ONE_DAY;
        self.counter += 1;
        Ok(())
    }

    fn is_year_set(&self) -> bool {
        self.counter >= 1
    }

    fn is_month_set(&self) -> bool {
        self.counter >= 2
    }

    fn is_day_set(&self) -> bool {
        self.counter >= 3
    }

    fn calc(&self) -> Result<u64> {
        Ok(self.time_stamp - ONE_MONTH - ONE_DAY - EIGHT_HOURS)
    }
}
