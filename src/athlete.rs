use std::cmp::Ordering;

#[derive(Copy, Clone, Debug, Eq)]
pub struct Athlete<'a> {
    pub score: u32,
    pub name: &'a str,
}

impl PartialEq for Athlete<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl PartialOrd for Athlete<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.score.cmp(&other.score))
    }
}

impl Ord for Athlete<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}
