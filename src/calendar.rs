use std::ops::Add;

use anyhow::bail;
use chrono::{Datelike, Duration, Local, NaiveDateTime};
use rusqlite::Connection;
use tokio::time;

use crate::{
  celcat::{auth::login, calendar::fetch},
  config::{Group, PUBLIC_PATH},
  db::{db_get_period, db_init, db_update_calendar},
  elogln,
  io::{generate_ics, generate_json, generate_png},
  logln,
  utils::date::get_week_bounds,
  CONFIG,
};

pub async fn update_calendar_task() {
  if CONFIG.groups.is_empty() {
    logln!("No group configured, calendar will not update.");
    return;
  }

  let mut interval = time::interval(time::Duration::from_secs(CONFIG.calendar_fetch_interval));
  loop {
    interval.tick().await;
    logln!();
    logln!("Updating calendar...");
    match update_calendar().await {
      Err(e) => elogln!("Calendar update failed:\n{e}"),
      _ => logln!("Calendar updated!"),
    };
  }
}

async fn update_calendar() -> anyhow::Result<()> {
  let mut db_con = db_init()?;

  let now = Local::now().naive_local();
  let weekday = now.weekday().num_days_from_monday();
  let reference = if weekday > 4 {
    now.add(Duration::days(i64::from(7 - weekday)))
  } else {
    now
  };

  let (start, _) = get_week_bounds(&reference);
  let (_, end) =
    get_week_bounds(&start.add(Duration::days(i64::from(7 * CONFIG.calendar_fetch_range))));
  logln!("- Fetching from {start} to {end}");

  for group in &CONFIG.groups {
    if let Err(err) = update_group(group, &mut db_con, &reference, &(start, end)).await {
      elogln!("Calendar update failed for group {}:\n{err}", group.name)
    }
  }

  let _ = db_con.close();

  Ok(())
}

async fn update_group(
  group: &Group,
  db_con: &mut Connection,
  reference: &NaiveDateTime,
  period: &(NaiveDateTime, NaiveDateTime),
) -> anyhow::Result<()> {
  logln!("- {}: Logging in to celcat...", group.name);
  let (client, student_id_extract) = login(&group.username, &group.password).await?;

  let Some(student_id) = group.student_id.as_ref().or(student_id_extract.as_ref()) else {
    bail!("Failed to find a student id");
  };

  logln!("  - Fetching {}...", group.name);
  let calendar = match fetch(&client, student_id, period).await {
    Ok(calendar) => {
      logln!("  - Updating database...");
      if let Err(err) = db_update_calendar(db_con, &group.name, &calendar, period) {
        elogln!("Failed to update database for {}:\n{err}", group.name);
      }
      Ok(calendar)
    }
    Err(err) => {
      elogln!(
        "Failed to fetch calendar for {}, using previous data:\n{err}",
        group.name
      );
      db_get_period(db_con, &group.name, period)
    }
  }?;

  logln!("  - Generating ics and json files...");
  generate_ics(&calendar, PUBLIC_PATH.join([&group.name, "ics"].join(".")))?;
  generate_json(&calendar, PUBLIC_PATH.join([&group.name, "json"].join(".")))?;

  static CATEGORIES: [&str; 3] = ["CM", "TD", "Examen"];
  let group_dir_path = PUBLIC_PATH.join(&group.name);

  for category in CATEGORIES {
    let events = &calendar
      .iter()
      .filter(|e| e.celcat.event_category.starts_with(category))
      .cloned()
      .collect();
    let lowercase_category = category.to_lowercase();

    generate_ics(
      events,
      group_dir_path.join([&lowercase_category, "ics"].join(".")),
    )?;
    generate_json(
      events,
      group_dir_path.join([&lowercase_category, "json"].join(".")),
    )?;
  }

  let other_events = &calendar
    .iter()
    .filter(|e| {
      CATEGORIES
        .iter()
        .all(|c| !e.celcat.event_category.starts_with(c))
    })
    .cloned()
    .collect();

  generate_ics(other_events, group_dir_path.join("autre.ics"))?;
  generate_json(other_events, group_dir_path.join("autre.json"))?;

  logln!("  - Generating png file");
  generate_png(
    &db_get_period(db_con, &group.name, &get_week_bounds(reference))?,
    &group.name,
  )?;

  Ok(())
}
