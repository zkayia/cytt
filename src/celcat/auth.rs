
use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::{Client, redirect::Policy};

use crate::{
  CONFIG,
  config::CELCAT_HOST,
  celcat::models::CelcatClient,
  utils::{string::join_strings, header_map::extract_cookies}
};


pub async fn login() -> anyhow::Result<CelcatClient> {
  
  let client = Client::builder()
    .https_only(true)
    .redirect(Policy::none())
    .build()?;
  
  let (token, token_cookies) = fetch_token(&client).await?;
  
  let response = client.post(CELCAT_HOST.to_owned() + "/calendar/LdapLogin/Logon")
    .header("cookie", &token_cookies)
    .form(&[
      ("Name", &CONFIG.celcat_username),
      ("Password", &CONFIG.celcat_password),
      ("__RequestVerificationToken", &token),
    ])
    .send()
    .await?;
  
  let status_code = response.status().as_u16();
  if !(200..400).contains(&status_code) {
    anyhow::bail!("Bad status code during login: {}", status_code);
  }

  return Ok(
    CelcatClient{
      client,
      cookies: join_strings(extract_cookies(response.headers()), &token_cookies, "; ")
    }
  );
}

async fn fetch_token(client: &Client) -> anyhow::Result<(String, String)> {
  
  let response = client.get(CELCAT_HOST.to_owned() + "/calendar/LdapLogin").send().await?;
  
  let status_code = response.status().as_u16();
  if !(200..400).contains(&status_code) {
    anyhow::bail!("Bad status code during token fetch: {}", status_code);
  }

  let cookies = extract_cookies(response.headers());

  static TOKEN_REG: Lazy<Regex> = Lazy::new(
    || Regex::new(r#"<input\s*name="__RequestVerificationToken"\s*type="hidden"\s*value="(.+)""#).unwrap(),
  );

  let response_text = response.text().await?;

  let Some(token) = TOKEN_REG.captures(&response_text)
    .and_then(|capture| capture.get(1))
    .map(|token| token.as_str().to_owned())
  else {
    anyhow::bail!("Token not found");
  };

  return Ok((token, cookies));
}
