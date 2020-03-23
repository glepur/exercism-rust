#[derive(Debug)]
pub struct HighScores<'a> {
  list: &'a [u32]
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
      HighScores{ list: scores }
    }

    pub fn scores(&self) -> &[u32] {
      self.list
    }

    pub fn latest(&self) -> Option<u32> {
      self.list.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
      self.list.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
      let mut vector = self.list.to_vec();
      vector.sort();
      vector.iter().rev().take(3).copied().collect()
    }
}
