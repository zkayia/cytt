
use std::fs;
use regex::RegexBuilder;


fn extract_version() -> anyhow::Result<String> {
  
  let changelog = fs::read_to_string("CHANGELOG.md")?;
  let version_reg = RegexBuilder::new(r#"^#\s*(\d+\.\d+\.\d+)"#).multi_line(true).build()?;

  return match version_reg.captures(&changelog) {
    Some(m) => match m.get(1) {
      Some(g) => Ok(g.as_str().to_owned()),
      None => Err(anyhow::anyhow!("No match group found"))
    },
    None => Err(anyhow::anyhow!("No match found"))
  };
}

fn main() {
  
  println!("cargo:rerun-if-changed=CHANGELOG.md");
  
  let version_res = extract_version();

  if version_res.is_err() {
    println!("cargo:warning=Failed to extract version: {}", version_res.as_ref().err().unwrap());
  }

  let version = version_res.unwrap_or("0.0.0".to_owned());

  println!("cargo:rustc-env=CYTT_VERSION={version}");
  println!("cargo:rustc-env=CARGO_PKG_VERSION={version}");
}
