
use std::ops::{Sub, Add};

use chrono::{Datelike, DateTime, Duration, Local, Timelike, Utc};


pub fn rfc3339_add_tz(date: &str) -> String {
  return if date.chars().count() < 20 {
    format!("{}{}", date, Local::now().format("%:z"))
  } else {
    date.to_owned()
  };
}

pub fn dt_from_rfc3339(date: &str) -> anyhow::Result<DateTime<Local>> {
  return Ok(DateTime::parse_from_rfc3339(date)?.with_timezone(&Local));
}

pub fn dt_to_ics(date: &DateTime<Local>) -> String {
  // return format!("{}", date.format("%Y%m%dT%H%M%S%z"));
  return format!("{}Z", date.with_timezone(&Utc).format("%Y%m%dT%H%M%S"));
}

pub fn dt_to_ics_day(date: &DateTime<Local>) -> String {
  return format!("{}Z", date.with_timezone(&Utc).format("%Y%m%dT"));
}

pub fn get_week_bounds(date: &DateTime<Local>) -> (DateTime<Local>, DateTime<Local>) {
  let midnight = get_start_of_day(date);
  let weekday = i64::from(midnight.weekday().num_days_from_monday());
  return (
    midnight.sub(Duration::days(weekday)),
    get_end_of_day(&midnight.add(Duration::days(6 - weekday))),
  );
}

// pub fn get_month_bounds(date: &DateTime<Local>) -> (DateTime<Local>, DateTime<Local>) {
//   let midnight = get_start_of_day(date);
//   let day = i64::from(midnight.day() - 1);
//   let start_of_month = midnight.sub(Duration::days(day));
//   return (
//     start_of_month,
//     get_end_of_day(
//       &midnight.add(Duration::days(i64::from(days_per_month(date.month(), is_leap_year(date.year()))) - day)),
//     ),
//   );
// }

pub fn get_start_of_day(date: &DateTime<Local>) -> DateTime<Local> {
  return date.sub(
    Duration::hours(i64::from(date.hour()))
    + Duration::minutes(i64::from(date.minute()))
    + Duration::seconds(i64::from(date.second()))
    + Duration::nanoseconds(i64::from(date.nanosecond())),
  );
}

pub fn get_end_of_day(date: &DateTime<Local>) -> DateTime<Local> {
  return date.add(
    Duration::hours(23)
    + Duration::minutes(59)
    + Duration::seconds(59)
    + Duration::nanoseconds(999999999),
  );
}

// pub fn is_leap_year(year: i32) -> bool {
//   return (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
// }

// pub fn days_per_month(month: u32, is_leap_year: bool) -> u32 {
//   return if month == 2 {
//     28 + (if is_leap_year {1} else {0})
//   } else {
//     31 - (month - 1) % 7 % 2
//   };
// }
