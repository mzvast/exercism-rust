#[derive(Debug)]
pub struct HighScores {
    data: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        // unimplemented!(
        //     "Construct a HighScores struct, given the scores: {:?}",
        //     scores
        // )
        Self {
            data: scores.to_owned(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        // unimplemented!("Return all the scores as a slice")
        &self.data
    }

    pub fn latest(&self) -> Option<u32> {
        // unimplemented!("Return the latest (last) score")
       self.data.last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
        // unimplemented!("Return the highest score")
        // https://users.rust-lang.org/t/beginner-idiomatic-way-to-convert-option-u8-to-option-u8/28240
        // option<&u32> to option<u32>
        self.data.iter().max().cloned()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        // unimplemented!("Return 3 highest scores")
        let len = self.data.len();
        let mut inst = self.data.clone();
        inst.sort_by(|a, b| b.cmp(a));
        inst.truncate(3);
        inst
    }
}
