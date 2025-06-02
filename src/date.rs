use chrono::{Datelike, NaiveDate};

pub fn gen_date() -> NaiveDate{
	let start = NaiveDate::from_ymd_opt(1900, 1, 1).unwrap();
	let end = NaiveDate::from_ymd_opt(2100, 12, 12).unwrap();
	
	let min_date = start.num_days_from_ce();
	let max_date = end.num_days_from_ce();
	
	let day_from_ce: i32 = rand::random_range(min_date..max_date);
	let date = NaiveDate::from_num_days_from_ce_opt(day_from_ce).unwrap();
	date
}

#[cfg(test)]
mod tests{
	use super::*;

	#[test]
	fn date_within_range(){
		let start = NaiveDate::from_ymd_opt(1900, 1, 1).unwrap();
		let end = NaiveDate::from_ymd_opt(2100, 12, 12).unwrap();

		let date = gen_date();

		assert!(date >= start);
		assert!(date <= end);
	}
}