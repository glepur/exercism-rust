pub fn verse(n: u32) -> String {
  match n {
    0 => "No more bottles of beer on the wall, no more bottles of beer.\n\
    Go to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
    1 => "1 bottle of beer on the wall, 1 bottle of beer.\n\
    Take it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
    _ => {
      let suffix = if n !=2 { "s" } else { "" };
      format!("{num} bottles of beer on the wall, {num} bottles of beer.\n\
      Take one down and pass it around, {num2} bottle{s} of beer on the wall.\n", num = n, num2= n-1, s = suffix)
    }
  }
}

pub fn sing(start: u32, end: u32) -> String {
  if start <= end {
    (start..=end).map(verse).collect::<Vec<String>>().join("\n")
  } else {
    (end..=start).rev().map(verse).collect::<Vec<String>>().join("\n")
  }
}
