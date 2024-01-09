
use std::{ops::Add, fs};

use chrono::{Local, Duration, Datelike};
use tokio::time;

use crate::{
  CONFIG,
  config::PUBLIC_PATH,
  celcat::{auth::login, calendar::fetch},
  db::{db_init, db_update_calendar, db_get_period},
  io::{generate_png, generate_ics, generate_json},
  utils::date::get_week_bounds,
  logln,
  elogln,
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
  
  logln!("- Logging in to celcat...");
  let client = login().await?;

  let now = Local::now();
  let weekday = now.weekday().num_days_from_monday();
  let reference = if weekday > 4 {now.add(Duration::days(i64::from(7 - weekday)))} else {now};
  
  let (start, _) = get_week_bounds(&reference);
  let (_, end) = get_week_bounds(&start.add(Duration::days(i64::from(7 * CONFIG.calendar_fetch_range))));
  logln!("- Fetching from {start} to {end}");
  
  for group in &CONFIG.groups {

    logln!("- Fetching {}...", group.name);
    let calendar = fetch(&client, &group.student_id, &start, &end).await?;

    logln!("- Adding {} to database...", group.name);
    db_update_calendar(&mut db_con, &group.name, &calendar)?;

    logln!("- Generating {}.ics...", group.name);
    generate_ics(&calendar, PUBLIC_PATH.join([&group.name, "ics"].join(".")))?;
    
    logln!("- Generating {}.json...", group.name);
    generate_json(&calendar, PUBLIC_PATH.join([&group.name, "json"].join(".")))?;
    
    let group_dir_path = PUBLIC_PATH.join(&group.name);
    
    logln!("- Creating group directory...");
    fs::create_dir_all(&group_dir_path)?;

    static CATEGORIES: [&str; 3] = ["CM", "TD", "Examen"];
    
    for category in CATEGORIES {

      let events = &calendar.iter().filter(|e| e.celcat.event_category.starts_with(category)).cloned().collect();
      let lowercase_category = category.to_lowercase();

      logln!("- Generating {}.ics...", lowercase_category);
      generate_ics(events, group_dir_path.join([&lowercase_category, "ics"].join(".")))?;

      logln!("- Generating {}.json...", lowercase_category);
      generate_json(events, group_dir_path.join([&lowercase_category, "json"].join(".")))?;
    }

    let other_events = &calendar.iter().filter(
      |e| CATEGORIES.iter().all(|c| !e.celcat.event_category.starts_with(c))
    ).cloned().collect();
    
    logln!("- Generating autre.ics...");
    generate_ics(other_events, group_dir_path.join("autre.ics"))?;

    logln!("- Generating autre.json...");
    generate_json(other_events, group_dir_path.join("autre.json"))?;

    logln!("- Generating {}.png", group.name);
    generate_png(
      &db_get_period(&db_con, &group.name, &get_week_bounds(&Local::now()))?,
      &group.name
    )?;
  }

  let _ = db_con.close();

  return Ok(());
}
