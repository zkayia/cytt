
use std::{
  fs,
  path::{Path, PathBuf},
  io::{self, BufWriter, Write},
  cmp::{max, min}
};

use askama::Template;
use chrono::{Local, Datelike, Timelike};
use image::{Rgb, RgbImage};
use imageproc::{
  drawing::{draw_text_mut, draw_filled_rect_mut, draw_hollow_rect_mut},
  rect::Rect
};
use rusttype::Scale;

use crate::{
  celcat::models::Event,
  config::PUBLIC_PATH,
  templates::{IndexHttpTemplate, GroupIcsTemplate},
  utils::{
    date::dt_to_ics,
    fs::copy_recursively,
    image::{load_font, calc_text_size, rgb_from_text, load_bold_font, fit_text_width},
    num::{u32_i32, u64_u32, i32_u32, i32_usize, f32_i32}
  }
};


pub fn copy_static_files() -> io::Result<()> {
  
  return copy_recursively(Path::new("assets").join("static"), PUBLIC_PATH.as_path());
}

pub fn generate_html() -> anyhow::Result<()> {
  
  fs::write(PUBLIC_PATH.join("index.html"), IndexHttpTemplate.render()?)?;
  
  return Ok(());
}

pub fn generate_ics(events: &Vec<Event>, path: PathBuf) -> anyhow::Result<()> {
  
  fs::write(
    path,
    GroupIcsTemplate{now: &dt_to_ics(&Local::now()), events}.render()?
  )?;

  return Ok(());
}

pub fn generate_json(events: &Vec<Event>, path: PathBuf) -> io::Result<()> {

  let file = fs::File::create(path)?;
  
  let mut writer = BufWriter::new(file);
  
  serde_json::to_writer(&mut writer, events)?;
  
  writer.flush()?;
  
  return Ok(());
}

pub fn generate_png(events: &Vec<Event>, group_name: &str) -> anyhow::Result<()> {

  static BACKGROUND_COLOR: Rgb<u8> = Rgb([54u8, 57u8, 63u8]);
  static BORDER_COLOR: Rgb<u8> = Rgb([32u8, 34u8, 37u8]);
  static TEXT_COLOR: Rgb<u8> = Rgb([255u8, 255u8, 255u8]);

  static TEXT_SCALE: Scale = Scale{x: 20f32, y: 20f32};
  static TEXT_SCALE_BIG: Scale = Scale{x: 26f32, y: 26f32};
  let text_font = load_font()?;
  let text_bold_font = load_bold_font()?;

  static SCHEDULES_ROW_HEIGHT: i32 = (1080 - 54) / 19 - 2;
  
  static DAYS: &[&str; 5] = &["Lundi", "Mardi", "Mercredi", "Jeudi", "Vendredi"];
  static DAY_COLUMN_WIDTH: i32 = (1920 - 100) / 5 - 2;
  
  let image_rect = Rect::at(0, 0).of_size(1920, 1080);
  
  let mut image = RgbImage::new(1920, 1080);

  draw_filled_rect_mut(&mut image, image_rect, BACKGROUND_COLOR);
  draw_hollow_rect_mut(&mut image, image_rect, BORDER_COLOR);
  draw_hollow_rect_mut(&mut image, Rect::at(1, 1).of_size(1920 - 2, 1080 - 2), BORDER_COLOR);

  let horaires_size = calc_text_size(&text_font, group_name, TEXT_SCALE_BIG);
  draw_text_mut(
    &mut image,
    TEXT_COLOR,
    98 / 2 - horaires_size.0 / 2,
    SCHEDULES_ROW_HEIGHT / 2 - horaires_size.1 / 2,
    TEXT_SCALE_BIG,
    &text_font,
    group_name,
  );
  
  for (i, pos) in (98..1920 - 2).step_by(i32_usize(DAY_COLUMN_WIDTH)? + 2).enumerate() {
    draw_filled_rect_mut(&mut image, Rect::at(pos, 0).of_size(2, 1080), BORDER_COLOR);
    
    let days_size = calc_text_size(&text_font, DAYS[i], TEXT_SCALE_BIG);
    draw_text_mut(
      &mut image,
      TEXT_COLOR,
      pos + 1 + DAY_COLUMN_WIDTH / 2 - days_size.0 / 2,
      54 / 2 - days_size.1 / 2,
      TEXT_SCALE_BIG,
      &text_font,
      DAYS[i],
    );
  }

  for (i, pos) in (0..).zip((54..1080).step_by(i32_usize(SCHEDULES_ROW_HEIGHT)? + 2)) {
    draw_filled_rect_mut(&mut image, Rect::at(0, pos).of_size(if i == 0 {1920} else {98}, 2), BORDER_COLOR);

    let time = format!("{:0width$}h{:0width$}", 8 + (i / 2), 30 * (i % 2), width = 2);
    let time_size = calc_text_size(&text_font, &time, TEXT_SCALE);
    draw_text_mut(&mut image, TEXT_COLOR, 98 / 2 - time_size.0 / 2, pos + 4, TEXT_SCALE, &text_font, &time);
  }

  for event in events {

    let event_text_color = rgb_from_text(&event.celcat.text_color)?;
    let start = event.celcat.start;

    let event_rect_y = max(
      56 + u32_i32(max(start.hour() - 8, 0) * 60 + start.minute())? * (SCHEDULES_ROW_HEIGHT + 2) / 30,
      56 + 2,
    );

    let event_rect = Rect::at(
      98 + 4 + (DAY_COLUMN_WIDTH + 2) * u32_i32(start.weekday().num_days_from_monday())?,
      event_rect_y,
    ).of_size(
      i32_u32(DAY_COLUMN_WIDTH)? - 4,
      match event.celcat.end {
        Some(end) => max(
          1,
          min(
            u64_u32(end.signed_duration_since(start).num_minutes().unsigned_abs())? * i32_u32(SCHEDULES_ROW_HEIGHT + 2)? / 30,
            1080 - 4 - i32_u32(event_rect_y)?
          ),
        ),
        None => 1080 - 4 - i32_u32(event_rect_y)?,
      },
    );

    draw_filled_rect_mut(&mut image, event_rect, rgb_from_text(&event.celcat.background_color)?);

    let event_info = [
      (
        &text_font,
        &TEXT_SCALE,
        &format!(
          "{} - {}",
          event.celcat.start.format("%Hh%M"),
          match event.celcat.end {
            Some(end) => end.format("%Hh%M").to_string(),
            None => "".to_owned(),
          },
        ),
      ),
      (
        &text_bold_font,
        &TEXT_SCALE_BIG,
        &format!("{} - {}", event.celcat.event_category, event.subject.as_ref().unwrap_or(&"".to_owned())),
      ),
      (&text_bold_font, &TEXT_SCALE_BIG, &event.teachers.join(", ")),
      (&text_font, &TEXT_SCALE_BIG, &event.classrooms.join(", ")),
    ];
    let mut event_info_top = event_rect.top() + 4;

    for (font, scale, text) in event_info {
      if !text.is_empty() && event_info_top + f32_i32(scale.y) < event_rect.bottom() {

        draw_text_mut(
          &mut image,
          event_text_color,
          event_rect.left() + 4,
          event_info_top,
          *scale,
          font,
          &text[..fit_text_width(font, text, *scale, DAY_COLUMN_WIDTH - 4 * 2)]
        );
        event_info_top += f32_i32(scale.y);
      }
    }
    
    draw_hollow_rect_mut(&mut image, event_rect, BORDER_COLOR)
  }

  image.save(PUBLIC_PATH.join([group_name, "png"].join(".")))?;

  return Ok(());
}
