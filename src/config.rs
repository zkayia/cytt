
use std::{
  env::var,
  fmt,
  fs,
  path::Path
};

use once_cell::sync::Lazy;

use crate::{CONFIG, elogln, logln};


pub static DB_SEP: &str = ";";
pub static CELCAT_HOST: &str = "https://services-web.cyu.fr";
pub static PUBLIC_PATH: Lazy<&Path> = Lazy::new(|| Path::new(&CONFIG.public_path));


#[derive(Clone, Debug)]
pub struct Group {
  pub username: String,
  pub password: String,
  pub name: String,
  pub display_name: Option<String>,
  pub student_id: Option<String>,
  pub gcal_id: Option<String>,
  pub gcal_id_cm: Option<String>,
  pub gcal_id_td: Option<String>,
  pub gcal_id_examen: Option<String>,
  pub gcal_id_autre: Option<String>,
}

impl fmt::Display for Group {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    
    writeln!(formatter, "Group(")?;
    writeln!(formatter, "      username: {}", self.username)?;
    writeln!(formatter, "      password: {}", self.password)?;
    writeln!(formatter, "      name: {}", self.name)?;
    writeln!(formatter, "      display_name: {:?}", self.name)?;
    writeln!(formatter, "      student_id: {:?}", self.student_id)?;
    writeln!(formatter, "      gcal_id: {:?}", self.gcal_id)?;
    writeln!(formatter, "      gcal_id_cm: {:?}", self.gcal_id_cm)?;
    writeln!(formatter, "      gcal_id_td: {:?}", self.gcal_id_td)?;
    writeln!(formatter, "      gcal_id_examen: {:?}", self.gcal_id_examen)?;
    writeln!(formatter, "      gcal_id_autre: {:?}", self.gcal_id_autre)?;
    write!(formatter, "    )")?;
    
    Ok(())
  }
}

#[derive(Clone, Debug)]
pub struct Config {
  pub groups: Vec<Group>,
  pub host: String,
  pub port: String,
  pub data_path: String,
  pub public_path: String,
  pub calendar_fetch_interval: u64,
  pub calendar_fetch_range: u8,
}

impl fmt::Display for Config {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {

    writeln!(formatter, "Config(")?;
    writeln!(formatter, "  groups: [")?;
    for group in &self.groups {
      writeln!(formatter, "    {},", group)?;
    }
    writeln!(formatter, "  ],")?;
    writeln!(formatter, "  host: {},", self.host)?;
    writeln!(formatter, "  port: {},", self.port)?;
    writeln!(formatter, "  data_path: {},", self.data_path)?;
    writeln!(formatter, "  public_path: {},", self.public_path)?;
    writeln!(formatter, "  calendar_fetch_interval: {},", self.calendar_fetch_interval)?;
    writeln!(formatter, "  calendar_fetch_range: {},", self.calendar_fetch_range)?;
    writeln!(formatter, ")")?;
    
    Ok(())
  }
}

impl Config {

  pub fn load() -> Config {

    logln!();
    logln!("Loading config...");

    let mut groups: Vec<Group> = vec![];
    let mut n: u8 = 0;
    while var(format!("CYTT_GROUP_{n}_NAME")).is_ok() {
      groups.push(
        Group{
          username: match var(format!("CYTT_GROUP_{n}_USERNAME")) {
            Ok(value) => value,
            Err(_) => {
              elogln!("Something went wrong while loading `group_{n}_username`, skipping this group.");
              n += 1;
              continue;
            } 
          },
          password: match var(format!("CYTT_GROUP_{n}_PASSWORD")) {
            Ok(value) => value,
            Err(_) => {
              elogln!("Something went wrong while loading `group_{n}_password`, skipping this group.");
              n += 1;
              continue;
            } 
          },
          name: match var(format!("CYTT_GROUP_{n}_NAME")) {
            Ok(value) => match value.chars().all(|e| matches!(e, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_')) {
              true => value,
              false => {
                elogln!("`group_{n}_name` does not match `[A-Za-z0-9-_]`, skipping this group.");
                n += 1;
                continue;
              }
            },
            Err(_) => {
              elogln!("Something went wrong while loading `group_{n}_name`, skipping this group.");
              n += 1;
              continue;
            }
          },
          display_name: var(format!("CYTT_GROUP_{n}_DISPLAY_NAME")).ok(),
          student_id: var(format!("CYTT_GROUP_{n}_STUDENTID")).ok(),
          gcal_id: var(format!("CYTT_GROUP_{n}_GCALID")).ok(),
          gcal_id_cm: var(format!("CYTT_GROUP_{n}_GCALID_CM")).ok(),
          gcal_id_td: var(format!("CYTT_GROUP_{n}_GCALID_TD")).ok(),
          gcal_id_examen: var(format!("CYTT_GROUP_{n}_GCALID_EXAMEN")).ok(),
          gcal_id_autre: var(format!("CYTT_GROUP_{n}_GCALID_AUTRE")).ok(),
        }
      );
      n += 1;
    }

    let config = Config{
      groups,
      host: var("CYTT_HOST").unwrap_or("127.0.0.1".to_owned()),
      port: var("CYTT_PORT").unwrap_or("8000".to_owned()),
      data_path: var("CYTT_DATA_PATH").unwrap_or("./data".to_owned()),
      public_path: var("CYTT_PUBLIC_PATH").unwrap_or("./public".to_owned()),
      calendar_fetch_interval: match var("CYTT_CALENDAR_FETCH_INTERVAL") {
        Ok(value) => value.parse::<u64>().unwrap_or(60 * 30),
        Err(_) => 60 * 30
      },
      calendar_fetch_range: match var("CYTT_CALENDAR_FETCH_RANGE") {
        Ok(value) => value.parse::<u8>().unwrap_or(10),
        Err(_) => 10
      },
    };

    if let Err(err) = fs::create_dir_all(Path::new(&config.data_path)) {
      logln!("Failed to create/access the data dir:\n{err}");
    }
    if let Err(err) = fs::create_dir_all(Path::new(&config.public_path)) {
      logln!("Failed to create/access the public dir:\n{err}");
    }

    logln!("Config loaded: {config}");
    
    config
  }
}
