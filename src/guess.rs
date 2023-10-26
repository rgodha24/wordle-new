use crate::{
    response::{Response, ResponseType},
    word::Word,
};

pub struct Guess {
    pub word: Word,
    pub mask: Response,
}

#[repr(transparent)]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct CachedGuess(pub u64);

impl From<(&Guess, &Word)> for CachedGuess {
    fn from((g, w): (&Guess, &Word)) -> Self {
        let mut cached: u64 = 0;
        cached += u64::from(w);

        cached += u64::from(&g.word) << 25;

        cached += u64::from(g.mask) << 50;

        Self(cached)
    }
}

impl Guess {
    pub fn matches(&self, word: &Word) -> bool {
        // Check if the guess would be possible to observe when `word` is the correct answer.
        // This is equivalent to
        //     ResponseType::compute(word, &self.word) == self.mask
        // without _necessarily_ computing the full mask for the tested word
        let mut used = [false; 5];

        // Check Green letters
        for (i, (a, g)) in word.iter().zip(self.word.iter()).enumerate() {
            if a == g {
                if self.mask[i] != ResponseType::Green {
                    return false;
                }
                used[i] = true;
            } else if self.mask[i] == ResponseType::Green {
                return false;
            }
        }

        // Check Misplaced letters
        for (g, e) in self.word.iter().zip(self.mask.iter()) {
            if *e == ResponseType::Green {
                continue;
            }
            if ResponseType::is_misplaced(*g, word, &mut used) != (*e == ResponseType::Yellow) {
                return false;
            }
        }

        // The rest will be all correctly Wrong letters
        true
    }
}
