use std::{fs, path::Path};

use chrono::{Datelike, NaiveDate};
use serde_json::json;

use super::Generator;

/// A date [`Generator`].
///
/// The default date range is from `1900-1-1` to `2100-12-31`.
///
/// # Example
/// ```
/// use chrono::NaiveDate;
/// use filler::DateGen;
///
/// let start = NaiveDate::from_ymd_opt(1900,1,1).unwrap();
/// let end = NaiveDate::from_ymd_opt(1900,1,1).unwrap();
/// let date_gen = DateGen::new();
/// ```
pub struct DateGen {
    /// The mimimum date that can be generated
    start: NaiveDate,
    /// The maximum date that can be generated
    end: NaiveDate,
}

impl DateGen {
    pub fn new() -> Self {
        Self {
            start: NaiveDate::from_ymd_opt(1900, 1, 1).unwrap(),
            end: NaiveDate::from_ymd_opt(2100, 12, 31).unwrap(),
        }
    }

    pub fn start(mut self, date: NaiveDate) -> Self {
        self.start = date;
        self
    }

    pub fn end(mut self, date: NaiveDate) -> Self {
        self.end = date;
        self
    }
}

impl Generator for DateGen {
    type Output = NaiveDate;

    fn generate(&self) -> Self::Output {
        let min_date = self.start.num_days_from_ce();
        let max_date = self.end.num_days_from_ce();

        let day_from_ce: i32 = rand::random_range(min_date..max_date);
        let date = NaiveDate::from_num_days_from_ce_opt(day_from_ce).unwrap();
        date
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;
    use std::fs::File;
    // TODO:
    // - End date less than start date

    #[test]
    fn default_date_range() {
        let date_gen = DateGen::new();
        let start = NaiveDate::from_ymd_opt(1900, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2100, 12, 31).unwrap();
        assert_eq!(date_gen.start, start);
        assert_eq!(date_gen.end, end);
    }

    #[test]
    fn date_within_range() {
        let start = NaiveDate::from_ymd_opt(2015, 10, 12).unwrap();
        let end = NaiveDate::from_ymd_opt(2018, 5, 12).unwrap();

        let date_gen = DateGen::new().start(start).end(end);

        let date = date_gen.generate();

        assert!(date >= start);
        assert!(date <= end);
    }

    #[test]
    fn write_date_to_json() -> crate::Result<()> {
        let num: u32 = rand::random();
        let path = format!("./temp/test-{num}.json");
        let date_gen = DateGen::new();
        let dates = date_gen.generate_many(10);
        date_gen.write_json(dates, &path)?;

        let file = File::open(&path)?;
        let data: Value = serde_json::from_reader(file)?;
        let dates = data.get("data").unwrap().as_array().unwrap();
        assert_eq!(dates.len(), 10);
        fs::remove_file(&path)?;

        Ok(())
    }
}
