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
        // if self.scores.is_empty() {
        //     return None;
        // }
        // Some(self.scores[self.scores.len() - 1])
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().copied()
        // if self.scores.is_empty() {
        //     return None;
        // }
        // let mut biggest: u32 = 0;
        // for i in self.scores {
        //     let i = *i;
        //     if i > biggest {
        //         biggest = i;
        //     }
        // }
        // Some(biggest)
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut sorted = Vec::from(self.scores);
        sorted.sort_by(|a, b| b.cmp(a));
        sorted.into_iter().take(3).collect()
    }
}
