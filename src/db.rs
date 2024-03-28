use std::path::Path;

use chrono::NaiveDateTime;
use rusqlite::{params, Connection, Result};

use crate::{celcat::models::Event, config::DB_SEP, CONFIG};

pub fn db_init() -> Result<Connection> {
  let db_con = Connection::open(Path::new(&CONFIG.data_path).join("cytt.db"))?;

  db_con.execute(
    "CREATE TABLE IF NOT EXISTS events (
      group_name       TEXT NOT NULL,
      event_id         TEXT NOT NULL PRIMARY KEY,
      start_date       TEXT NOT NULL,
      end_date         TEXT,
      all_day          INTEGER NOT NULL,
      description      TEXT NOT NULL,
      background_color TEXT NOT NULL,
      text_color       TEXT NOT NULL,
      event_category   TEXT NOT NULL,
      teachers         TEXT NOT NULL,
      classrooms       TEXT NOT NULL,
      subject          TEXT
    )",
    (),
  )?;

  Ok(db_con)
}

pub fn db_update_calendar(
  db_con: &mut Connection,
  group_name: &str,
  calendar: &Vec<Event>,
  period: &(NaiveDateTime, NaiveDateTime),
) -> Result<()> {
  if calendar.is_empty() {
    return Ok(());
  }

  let tx = db_con.transaction()?;

  {
    tx.execute(
      "DELETE FROM events WHERE group_name = ?1 AND start_date BETWEEN ?2 AND ?3",
      params![
        group_name,
        period.0.format("%Y-%m-%dT%H:%M:%S").to_string(),
        period.1.format("%Y-%m-%dT%H:%M:%S").to_string(),
      ],
    )?;

    let mut insert = tx.prepare(
      "INSERT OR REPLACE INTO events (
        group_name,
        event_id,
        start_date,
        end_date,
        all_day,
        description,
        background_color,
        text_color,
        event_category,
        teachers,
        classrooms,
        subject
      ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
    )?;

    for event in calendar {
      insert.execute(params![
        group_name,
        event.celcat.id,
        event.celcat.start.format("%Y-%m-%dT%H:%M:%S").to_string(),
        match event.celcat.end {
          Some(value) => value.format("%Y-%m-%dT%H:%M:%S").to_string(),
          None => "NULL".to_owned(),
        },
        event.celcat.all_day,
        event.celcat.description,
        event.celcat.background_color,
        event.celcat.text_color,
        event.celcat.event_category,
        event.teachers.join(DB_SEP),
        event.classrooms.join(DB_SEP),
        event.subject.as_deref().unwrap_or("NULL"),
      ])?;
    }
  }

  tx.commit()?;

  Ok(())
}

pub fn db_get_all(db_con: &Connection, group_name: &str) -> anyhow::Result<Vec<Event>> {
  let mut select = db_con.prepare("SELECT * FROM events WHERE group_name = ?1")?;

  let results = select.query_and_then([group_name], Event::from_sql_row)?;

  Ok(results.filter_map(|e| e.ok()).collect())
}

pub fn db_get_period(
  db_con: &Connection,
  group_name: &str,
  period: &(NaiveDateTime, NaiveDateTime),
) -> anyhow::Result<Vec<Event>> {
  let mut select = db_con
    .prepare("SELECT * FROM events WHERE group_name = ?1 AND start_date BETWEEN ?2 AND ?3")?;

  let results = select.query_and_then(
    (
      group_name,
      period.0.format("%Y-%m-%dT%H:%M:%S").to_string(),
      period.1.format("%Y-%m-%dT%H:%M:%S").to_string(),
    ),
    Event::from_sql_row,
  )?;

  Ok(results.filter_map(|e| e.ok()).collect())
}
