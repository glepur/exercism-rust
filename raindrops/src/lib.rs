pub fn raindrops(n: u32) -> String {
  let v : String = vec![(3, "Pling"), (5, "Plang"), (7, "Plong")]
    .iter()
    .fold(String::new(), |mut acc, t| {
      if n % t.0 == 0 {
        acc.push_str(t.1)
      }
      acc
    });
  match v.len() {
    0 => n.to_string(),
    _ => v
  }
}
