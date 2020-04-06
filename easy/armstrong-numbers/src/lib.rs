pub fn is_armstrong_number(num: u32) -> bool {
  let string = num.to_string();
  let result = string
    .chars()
    .map(|character|
      character
        .to_digit(10)
        .unwrap()
        .pow(string.len() as u32)
    )
    .sum();
  num == result
}
