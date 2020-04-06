use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
  string.chars().fold(Vec::new(), |mut v, c| {
    if is_closing(&v, &c) {
      v.pop();
    } else if "{}[]()".contains(c) {
      v.push(c);
    }

    v
  }).len() == 0
}

fn is_closing(v: &Vec::<char>, c: &char) -> bool {
  let match_map: HashMap<char, char> = [('}', '{'), (']', '['), (')', '(')]
    .iter().cloned().collect();

  v.len() > 0
  && match_map.contains_key(&c)
  && &v[v.len() - 1] == match_map.get(&c).unwrap()
}