
use chrono::{DateTime, Local};

use crate::{
  config::CELCAT_HOST,
  celcat::models::{CelcatEvent, Event, CelcatClient}
};


pub async fn fetch(
  client: &CelcatClient,
  student_id: &str,
  period: &(DateTime<Local>, DateTime<Local>)
) -> anyhow::Result<Vec<Event>> {
  
  let response = client.client.post(CELCAT_HOST.to_owned() + "/calendar/Home/GetCalendarData")
    .header("cookie", &client.cookies)
    .form(&[
      ("start", format!("{}", period.0.format("%Y-%m-%d"))),
      ("end", format!("{}", period.1.format("%Y-%m-%d"))),
      ("resType", "104".to_owned()),
      ("calView", "month".to_owned()),
      ("federationIds[]", student_id.to_owned()),
      ("colourScheme", "3".to_owned()),
    ])
    .send()
    .await?;

  let status_code = response.status().as_u16();
  if !(200..400).contains(&status_code) {
    anyhow::bail!("Bad status code during calendar fetch: {}", status_code);
  }

  return Ok(
    response.json::<Vec<CelcatEvent>>()
      .await?
      .iter()
      .map(Event::from_celcat_event)
      .collect()
  );
}