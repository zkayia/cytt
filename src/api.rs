
use axum::{extract::Path, Json, http::StatusCode};
use chrono::Local;

use crate::{
  celcat::models::Event,
  db::{db_init, db_get_all, db_get_period},
  utils::date::get_week_bounds
};


pub async fn api_timetable_all(Path(group): Path<String>) -> Result<Json<Vec<Event>>, StatusCode> {
  
  let db_con = db_init().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
  
  let Ok(timetable) = db_get_all(&db_con, &group) else {
    let _ = db_con.close();
    return Err(StatusCode::INTERNAL_SERVER_ERROR);
  };

  let _ = db_con.close();

  Ok(Json(timetable))
}

pub async fn api_timetable_week(Path(group): Path<String>) -> Result<Json<Vec<Event>>, StatusCode> {
  
  let db_con = db_init().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
  
  let week = get_week_bounds(&Local::now().naive_local());

  let Ok(current_week) = db_get_period(&db_con, &group, &week) else {
    let _ = db_con.close();
    return Err(StatusCode::INTERNAL_SERVER_ERROR);
  };

  let _ = db_con.close();

  Ok(Json(current_week))
}