pub fn factors(n: u64) -> Vec<u64> {
  let mut dividend = n;
  let mut factors = vec!();
  let mut divisor = 2;
  while dividend != 1 {
    if dividend % divisor == 0 {
      factors.push(divisor);
      dividend = dividend / divisor;
    } else {
      divisor += 1;
    }
  }
  factors
}
