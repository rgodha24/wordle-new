use std::borrow::Cow;

use crate::{
    response::{Response, ResponseType},
    word::Word,
};

pub struct Guess<'a> {
    pub word: Cow<'a, Word>,
    pub mask: Response,
}

impl Guess<'_> {
    pub fn matches(&self, word: &Word) -> bool {
        // Check if the guess would be possible to observe when `word` is the correct answer.
        // This is equivalent to
        //     ResponseType::compute(word, &self.word) == self.mask
        // without _necessarily_ computing the full mask for the tested word
        assert_eq!(word.len(), 5);
        assert_eq!(self.word.len(), 5);
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
