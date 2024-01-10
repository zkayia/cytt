
mod api;
mod calendar;
mod celcat;
mod config;
mod db;
mod io;
mod utils;
mod templates;

use api::{api_timetable_all, api_timetable_week};
use axum::{
  Router,
  routing::get,
  http::{header, HeaderValue}
};
use calendar::update_calendar_task;
use config::{Config, PUBLIC_PATH};
use io::{setup_public_dir, generate_html};
use once_cell::sync::Lazy;
use tokio::{net::TcpListener, task};
use tower::ServiceBuilder;
use tower_http::{services::ServeDir, trace::TraceLayer, set_header::SetResponseHeaderLayer};


pub static CONFIG: Lazy<Config> = Lazy::new(Config::load);

#[tokio::main]
async fn main() {

  logln!(
    "cytt {}{}",
    env!("CARGO_PKG_VERSION"),
    if option_env!("CYTT_IS_DOCKER").is_some() {" Docker"} else {""}
  );
  
  // initialise the config now so it doesnt break the log flow
  let _ = CONFIG.calendar_fetch_range;
  
  logln!();
  logln!("Setting up public dir...");
  match setup_public_dir() {
    Ok(_) => logln!("Public dir ready!"),
    Err(e) => elogln!("Public dir setup failed:\n{e}")
  }

  logln!();
  logln!("Generating HTML...");
  match generate_html() {
    Ok(_) => logln!("HTML generated!"),
    Err(e) => elogln!("HTML generation failed:\n{e}")
  }
  
  let cache_control = match HeaderValue::from_str(format!("max-age={}", CONFIG.calendar_fetch_interval).as_str()) {
    Ok(value) => value,
    Err(_) => HeaderValue::from_static("max-age=600"),
  };

  let serve_dir = ServiceBuilder::new()
    .layer(SetResponseHeaderLayer::overriding(header::CACHE_CONTROL, cache_control))
    .service(ServeDir::new(PUBLIC_PATH.as_path()));

  let router = Router::new()
    .route("/api/:group/timetable/all", get(api_timetable_all))
    .route("/api/:group/timetable/week", get(api_timetable_week))
    .fallback_service(serve_dir);
  
  let address = format!("{}:{}", CONFIG.host, CONFIG.port);

  logln!();
  logln!("Listening on {address}");
  
  let updates = task::spawn(update_calendar_task());
  
  axum::serve(
    TcpListener::bind(&address).await.unwrap(),
    router.layer(TraceLayer::new_for_http())
  ).await.unwrap();
  
  let _ = updates.await;
}
