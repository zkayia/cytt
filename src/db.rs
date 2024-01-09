
use std::path::Path;

use chrono::{DateTime, Local, SecondsFormat};
use rusqlite::{Connection, Result, params};

use crate::{
  CONFIG,
  config::DB_SEP,
  celcat::models::Event,
};


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
  
  return Ok(db_con);
}

pub fn db_update_calendar(db_con: &mut Connection, group_name: &str, calendar: &Vec<Event>) -> Result<()> {
  if calendar.is_empty() {
    return Ok(());
  }
  
  let tx = db_con.transaction()?;

  {
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
      ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)"
    )?;

    for event in calendar {
      insert.execute(params![
        group_name,
        event.celcat.id,
        event.celcat.start.to_rfc3339_opts(SecondsFormat::Secs, false),
        match event.celcat.end {
          Some(value) => value.to_rfc3339_opts(SecondsFormat::Secs, false),
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

  return Ok(());
}

pub fn db_get_all(db_con: &Connection, group_name: &str) -> anyhow::Result<Vec<Event>> {

  let mut select = db_con.prepare("SELECT * FROM events WHERE group_name = ?1")?;

  let results = select.query_and_then([group_name], Event::from_sql_row)?;

  return Ok(results.filter_map(|e| e.ok()).collect());
}

pub fn db_get_period(
  db_con: &Connection,
  group_name: &str,
  period: &(DateTime<Local>, DateTime<Local>)
) -> anyhow::Result<Vec<Event>> {

  let mut select = db_con.prepare("SELECT * FROM events WHERE group_name = ?1 AND start_date BETWEEN ?2 AND ?3")?;

  let results = select.query_and_then(
    (
      group_name,
      period.0.to_rfc3339_opts(SecondsFormat::Secs, false),
      period.1.to_rfc3339_opts(SecondsFormat::Secs, false),
    ),
    Event::from_sql_row,
  )?;

  return Ok(results.filter_map(|e| e.ok()).collect());
}
