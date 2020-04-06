use rand::Rng;

pub fn private_key(p: u64) -> u64 {
  rand::thread_rng().gen_range(2, p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
  mod_exp(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
  mod_exp(b_pub, a, p)
}

fn mod_exp(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
  if modulus == 1 {
    return 0;
  }
  let mut result = 1;
  base = base % modulus;
  while exp > 0 {
    if exp % 2 == 1 {
      result = result * base % modulus;
    }
    exp = exp >> 1;
    base = base * base % modulus
  }
  result
}
