

pub fn join_strings(string: String, other: &str, sep: &str) -> String {
  if other.is_empty() {
    return string;
  }
  if string.is_empty() {
    return other.to_owned();
  }
  return string + sep + other;
}