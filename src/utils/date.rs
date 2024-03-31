use std::ops::{Add, Sub};

use chrono::{DateTime, Datelike, Duration, NaiveDate, NaiveDateTime, Timelike, Utc};

pub fn dt_from_rfc3339(date: &str) -> anyhow::Result<NaiveDateTime> {
  Ok(NaiveDateTime::parse_from_str(date, "%Y-%m-%dT%H:%M:%S")?)
}

pub fn dt_to_ics(date: &NaiveDateTime) -> String {
  // return format!("{}", date.format("%Y%m%dT%H%M%S%z"));
  format!("{}Z", get_utc(date).format("%Y%m%dT%H%M%S"))
}

pub fn dt_to_ics_day(date: &NaiveDateTime) -> String {
  format!("{}Z", get_utc(date).format("%Y%m%dT"))
}

pub fn get_utc(date: &NaiveDateTime) -> DateTime<Utc> {
  let summer_start = last_sunday_of_month(date.year(), 3)
    .and_hms_opt(2, 0, 0)
    .unwrap();
  // Will give the wrong time during the hour just before the end but whatever
  let summer_end = last_sunday_of_month(date.year(), 10)
    .and_hms_opt(2, 0, 0)
    .unwrap();
  let is_summer_time = summer_start <= *date && date < &summer_end;
  date
    .sub(Duration::hours(if is_summer_time { 2 } else { 1 }))
    .and_utc()
}

pub fn last_sunday_of_month(year: i32, month: u32) -> NaiveDate {
  let last_day_of_month =
    NaiveDate::from_ymd_opt(year, month, days_per_month(year, month)).unwrap();
  last_day_of_month.sub(Duration::days(i64::from(
    last_day_of_month.weekday().num_days_from_sunday(),
  )))
}

pub fn get_week_bounds(date: &NaiveDateTime) -> (NaiveDateTime, NaiveDateTime) {
  let midnight = get_start_of_day(date);
  let weekday = i64::from(midnight.weekday().num_days_from_monday());
  (
    midnight.sub(Duration::days(weekday)),
    get_end_of_day(&midnight.add(Duration::days(6 - weekday))),
  )
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

pub fn get_start_of_day(date: &NaiveDateTime) -> NaiveDateTime {
  date.sub(
    Duration::hours(i64::from(date.hour()))
      + Duration::minutes(i64::from(date.minute()))
      + Duration::seconds(i64::from(date.second()))
      + Duration::nanoseconds(i64::from(date.nanosecond())),
  )
}

pub fn get_end_of_day(date: &NaiveDateTime) -> NaiveDateTime {
  date.add(
    Duration::hours(23)
      + Duration::minutes(59)
      + Duration::seconds(59)
      + Duration::nanoseconds(999999999),
  )
}

pub fn is_leap_year(year: i32) -> bool {
  (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

pub fn days_per_month(year: i32, month: u32) -> u32 {
  if month == 2 {
    28 + (if is_leap_year(year) { 1 } else { 0 })
  } else {
    31 - (month - 1) % 7 % 2
  }
}
