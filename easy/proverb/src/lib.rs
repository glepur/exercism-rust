pub fn build_proverb(list: &[&str]) -> String {
  if list.is_empty() {
    return String::new();
  }
  list.iter().zip(list.iter().skip(1)).map(|(f, s)| {
    format!("For want of a {} the {} was lost.", f, s)
  })
  .chain(std::iter::once(format!("And all for the want of a {}.", list[0])))
  .collect::<Vec<_>>()
  .join("\n")
}
