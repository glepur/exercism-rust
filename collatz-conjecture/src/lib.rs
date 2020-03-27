pub fn collatz(n: u64) -> Option<u64> {
  match n {
    0 => None,
    _ => {
      let mut num = n;
      let mut steps = 0;
      while num != 1 {
        steps += 1;
        num = if num % 2 == 0 {
          num / 2
        } else {
          num * 3 + 1
        }
      }
      Some(steps)
    }
  }
}
