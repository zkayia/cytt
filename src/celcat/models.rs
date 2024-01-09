
use chrono::{DateTime, Local};
use html_escape::decode_html_entities;
use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::Client;
use rusqlite::Row;
use serde::{de::Error, Deserialize, Serialize, Deserializer};

use crate::{config::DB_SEP, utils::date::{dt_from_rfc3339, rfc3339_add_tz}};


#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CelcatEvent {
	pub id: String,
  #[serde(deserialize_with = "from_rfc3339")]
	pub start: DateTime<Local>,
  #[serde(deserialize_with = "maybe_from_rfc3339")]
	pub end: Option<DateTime<Local>>,
	pub all_day: bool,
	pub description: String,
	pub background_color: String,
	pub text_color: String,
	pub event_category: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Event {
	pub celcat: CelcatEvent,
  pub teachers: Vec<String>,
  pub classrooms: Vec<String>,
  pub subject: Option<String>,
}

#[derive(Clone)]
pub struct CelcatClient {
  pub client: Client,
  pub cookies: String,
}

fn maybe_from_rfc3339<'de, D>(deserializer: D) -> Result<Option<DateTime<Local>>, D::Error>
where
  D: Deserializer<'de>,
{
  return match Option::<&str>::deserialize(deserializer)? {
    Some(date) => dt_from_rfc3339(&rfc3339_add_tz(date)) 
      .map(Some)
      .map_err(D::Error::custom),
    None => Ok(None)
  };
}

fn from_rfc3339<'de, D>(deserializer: D) -> Result<DateTime<Local>, D::Error>
where
  D: Deserializer<'de>,
{
  return dt_from_rfc3339(
    &rfc3339_add_tz(Deserialize::deserialize(deserializer)?)
  ).map_err(D::Error::custom);
}

impl Event {

  pub fn from_celcat_event(celcat_event: &CelcatEvent) -> Event {

    let decoded = decode_html_entities(celcat_event.description.trim());
    let parts = decoded.split("\r\n\r\n<br />\r\n\r\n").map(|e| e.trim());
    let parts_length = parts.to_owned().count();

    let mut teachers: Vec<String> = vec![];
    let mut classrooms: Vec<String> = vec![];
    let mut subject: Option<String> = None;

    static TEACHER_REG: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^([A-Z -]|(<br \/>))+$"#).unwrap());
    static TDCM_REG: Lazy<Regex> = Lazy::new(|| Regex::new("(TD|CM)").unwrap());
    static TDCM_START_REG: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^(TD|CM)\s"#).unwrap());
    static TDCM_WORD_REG: Lazy<Regex> = Lazy::new(|| Regex::new(r#"(\s|^)(TD|CM)(\s|$)"#).unwrap());
    static CLASSROOM_REG: Lazy<Regex> = Lazy::new(|| Regex::new(r#"[AE]\d{3}"#).unwrap());

    for (i, part) in parts.enumerate() {

      if part.starts_with("PAU ") {
        for classroom in CLASSROOM_REG.find_iter(part) {
          classrooms.push(classroom.as_str().to_owned());
        }
        continue;
      }
      
      if !TDCM_WORD_REG.is_match(part) && (part.starts_with("Vac_tempo_CYTECH") || TEACHER_REG.is_match(part)) {
        for teacher in part.split("<br />") {
          teachers.push(teacher.to_owned());
        }
      }
      
      if TDCM_WORD_REG.is_match(part) {
        if TDCM_START_REG.is_match(part) {
          if let Some(tdcm) = TDCM_REG.find_iter(part).last() {
            subject = Some(part[tdcm.end()..].to_owned());
          }
        } else if part.contains(' ') {
          if let Some(tdcm) = TDCM_REG.find(part) {
            subject = Some(part[..(tdcm.start()-1)].to_owned());
          }
        }
      }

      if ((i == 1 && parts_length < 4) || i == 3) && subject.is_none() && celcat_event.event_category == "Indisponibilité" {
        subject = Some(part.to_owned());
      }
      if i == 2 && subject.is_none() && parts_length > 4 {
        subject = match TDCM_REG.find(part) {
          Some(tdcm) => Some(part[..tdcm.start()].to_owned()),
          None => Some(part.to_owned()),
        }
      }
    }

    return Event{
      celcat: CelcatEvent{
        id: celcat_event.id.to_owned(),
        start: celcat_event.start.to_owned(),
        end: celcat_event.end.to_owned(),
        all_day: celcat_event.all_day,
        description: decoded.to_string(),
        background_color: celcat_event.background_color.to_owned(),
        text_color: celcat_event.text_color.to_owned(),
        event_category: celcat_event.event_category.to_owned(),
      },
      teachers,
      classrooms,
      subject: subject.map(|e| e.trim().to_owned()),
    };
  }

  pub fn from_sql_row(row: &Row) -> anyhow::Result<Event> {
    return Ok(Event{
      celcat: CelcatEvent{
        id: row.get(1)?,
        start: dt_from_rfc3339(&row.get::<usize, String>(2)?)?,
        end: if row.get::<usize, String>(3)? == "NULL" {
          None
        } else {
          Some(dt_from_rfc3339(&row.get::<usize, String>(3)?)?)
        },
        all_day: row.get(4)?,
        description: row.get(5)?,
        background_color: row.get(6)?,
        text_color: row.get(7)?,
        event_category: row.get(8)?,
      },
      teachers: (row.get::<usize, String>(9)?)
        .split(DB_SEP)
        .filter(|e| !e.is_empty())
        .map(String::from)
        .collect(),
      classrooms: (row.get::<usize, String>(10)?)
        .split(DB_SEP)
        .filter(|e| !e.is_empty())
        .map(String::from)
        .collect(),
      subject: if row.get::<usize, String>(11)? == "NULL" {None} else {Some(row.get(11)?)},
    })
  }
}
