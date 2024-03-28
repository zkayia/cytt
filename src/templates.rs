use askama::Template;

use crate::celcat::models::Event;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexHttpTemplate;

#[derive(Template)]
#[template(path = "group.ics", escape = "none")]
pub struct GroupIcsTemplate<'t> {
  pub now: &'t str,
  pub events: &'t Vec<Event>,
}
