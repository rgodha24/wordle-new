use crate::{guess::Guess, response::Response};
use std::{
    borrow::Cow,
    collections::HashSet,
    fmt::Display,
    ops::{Deref, Index},
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Word([u8; 5]);
#[derive(Debug, Clone)]
pub struct Letter {
    is: HashSet<u8>,
}

fn test_answer(guess: &Word, answer: &Word, response: Response) -> bool {
    let guess = Guess {
        word: Cow::Borrowed(guess),
        mask: response,
    };

    guess.matches(answer)
}

impl Word {
    /// lower is better
    pub fn score_new(&self, answers: &HashSet<Word>, run_number: &usize) -> usize {
        let mut score = 0;
        for answer in answers {
            let response = Response::from_answer(self, answer);
            score += answers
                .iter()
                .filter(|ans| test_answer(self, *ans, response))
                .count();
        }

        if answers.contains(self) {
            score -= *run_number;
        }

        score
    }
}

impl Letter {
    pub fn remove_choice(&mut self, c: u8) -> bool {
        self.is.remove(&c)
    }
    pub fn set_choice(&mut self, c: u8) {
        self.is = [c].into_iter().collect();
    }
}

impl Index<usize> for Word {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Deref for Word {
    type Target = [u8; 5];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for Letter {
    type Target = HashSet<u8>;

    fn deref(&self) -> &Self::Target {
        &self.is
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &c in self.0.iter() {
            write!(f, "{}", char::from(c))?;
        }
        Ok(())
    }
}

impl From<&str> for Word {
    fn from(value: &str) -> Self {
        if value.len() != 5 {
            panic!("Word {value} isn't 5 characters long");
        }

        let mut chars = [b' '; 5];
        for (i, c) in value.bytes().enumerate() {
            chars[i] = c;
        }
        Self(chars)
    }
}

impl Default for Letter {
    fn default() -> Self {
        Self {
            is: (b'a'..=b'z').collect(),
        }
    }
}
impl Display for Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for &c in self.is.iter() {
            let c: char = c.into();
            s.push(c);
        }
        write!(f, "{}", s)
    }
}
