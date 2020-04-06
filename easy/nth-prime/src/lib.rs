pub fn nth(n: u32) -> u32 {
  let mut i = 0;
  let mut num = 2;
  while i < n {
    num += 1;
    if is_prime(num) {
      i += 1;
    }
  }
  num
}

fn is_prime(n: u32) -> bool {
  let n_half = (n / 2) + 1;
  let mut result = true;
  for x in 2..n_half {
    if x != n && n % x == 0 {
      result = false;
      break;
    }
  }
  result
}