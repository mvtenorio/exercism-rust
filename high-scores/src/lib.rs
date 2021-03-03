#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        let mut c: Vec<u32> = self.scores.iter().copied().collect();
        c.sort_unstable();
        c.last().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut c: Vec<u32> = self.scores.iter().copied().collect();
        c.sort_unstable();
        c.iter().rev().take(3).copied().collect()
    }
}