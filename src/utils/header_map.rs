
use reqwest::header::HeaderMap;

use crate::utils::string::join_strings;


pub fn extract_cookies(headers: &HeaderMap) -> String {
  let mut cookies: String = "".to_owned();
  for header in headers {
    if header.0.as_str() == "set-cookie" {
      let Ok(value) = header.1.to_str() else {
        continue;
      };
      let Some(end) = value.find(';') else {
        continue;
      };
      cookies = join_strings(cookies, &value[..end], "; ");
    }
  }
  cookies
}