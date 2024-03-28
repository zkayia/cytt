
use std::num::ParseIntError;

use anyhow::anyhow;
use image::Rgb;
use rusttype::{Font, point, Scale};


pub fn load_font() -> anyhow::Result<Font<'static>> {
  match Font::try_from_bytes(include_bytes!("../../assets/fonts/poppins-semi-bold.ttf")) {
    Some(font) => Ok(font),
    None => Err(anyhow!("Failed to generate png: Font not found")),
  }
}

pub fn load_bold_font() -> anyhow::Result<Font<'static>> {
  match Font::try_from_bytes(include_bytes!("../../assets/fonts/poppins-bold.ttf")) {
    Some(font) => Ok(font),
    None => Err(anyhow!("Failed to generate png: Font not found")),
  }
}

// https://stackoverflow.com/questions/68151488/rusttype-get-text-width-for-font
pub fn calc_text_size(font: &Font, text: &str, scale: Scale) -> (i32, i32) {
  let width = font
    .layout(text, scale, point(0.0, 0.0))
    .last()
    .map(|g| g.position().x + g.unpositioned().h_metrics().advance_width)
    .unwrap_or(0.0);

  let v_metrics = font.v_metrics(scale);
  let height = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;

  (width.round() as i32, height.round() as i32)
}

pub fn fit_text_width(font: &Font, text: &str, scale: Scale, max_width: i32) -> usize {
  if !text.is_ascii() {
    return text.len();
  }

  let mut width = calc_text_size(font, text, scale).0;
  let mut len = text.len();
  
  while width > max_width {
    len -= 1;
    width = calc_text_size(font, &text[..len], scale).0;
  }

  len
}

pub fn rgb_from_text(color: &str) -> Result<Rgb<u8>, ParseIntError> {
  
  let start = if color.starts_with('#') {1} else {0};
  let mut u8_color = [0u8, 0u8, 0u8];

  for (i, pos) in (start..7).step_by(2).enumerate() {
    u8_color[i] = u8::from_str_radix(&color[pos..pos + 2], 16)?;
  }

  Ok(Rgb(u8_color))
}
