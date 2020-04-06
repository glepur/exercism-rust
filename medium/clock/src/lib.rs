use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
  hours: i32,
  minutes: i32,
}

impl fmt::Display for Clock {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:02}:{:02}", self.hours, self.minutes)
  }
}

impl Clock {
  pub fn new(hours: i32, minutes: i32) -> Self {
    Clock { hours, minutes }.normalize()
  }

  pub fn add_minutes(&self, minutes: i32) -> Self {
    Clock {
      hours: self.hours,
      minutes: self.minutes + minutes,
    }
    .normalize()
  }

  fn normalize(&self) -> Self {
    let (minutes, hours_offset) = euclidean_division(self.minutes, 60);
    let (hours, _) = euclidean_division(self.hours + hours_offset, 24);

    Clock { hours, minutes }
  }
}

fn euclidean_division(n: i32, m: i32) -> (i32, i32) {
  (n.rem_euclid(m), n.div_euclid(m))
}
