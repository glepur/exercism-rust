use regex::Regex;

pub fn reply(message: &str) -> &str {
  let yell = Regex::new(r"^[^a-z]*[A-Z]+[^a-z]*$").unwrap();
  let m = message.trim();

  if yell.is_match(m) && m.ends_with('?') {
    return "Calm down, I know what I'm doing!"
  } else if m.ends_with("?") {
    return "Sure."
  } else if yell.is_match(m) {
    return "Whoa, chill out!"
  } else if m.is_empty() {
    return "Fine. Be that way!"
  }
  "Whatever."
}
