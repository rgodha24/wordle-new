use itertools::Itertools;
use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    ops::Deref,
};

fn main() {
    let board = Board::default();

    let ok = include_str!("../ok.json");
    let answers = include_str!("../answers.json");
    let ok: HashSet<&str> = serde_json::from_str(&ok).unwrap();
    let answers: HashSet<&str> = serde_json::from_str(&answers).unwrap();

    let mut answers: HashSet<Word> = answers.into_iter().map(|w| w.into()).collect();
    let mut ok: HashSet<Word> = ok.into_iter().map(|w| w.into()).collect();
    let board = Board::default();

    for _ in 0..5 {
        let mut answer_letters: [HashMap<char, usize>; 5] = Default::default();

        for answer in answers.iter() {
            for (i, c) in answer.0.iter().enumerate() {
                *answer_letters[i].entry(*c).or_default() += 1;
            }
        }

        let best_chars: [Vec<char>; 5] = answer_letters
            .iter()
            .map(|letters| {
                letters
                    .iter()
                    .sorted_by_key(|(_, count)| *count)
                    .map(|(c, _)| *c)
                    .rev()
                    .collect_vec()
            })
            .collect_vec()
            .try_into()
            .unwrap();

        let best_choices = ok
            .par_iter()
            .map(|w| (w, w.score(&best_chars)))
            .min_by_key(|x| x.1)
            .expect("best word exists")
            .0;

        println!("{}", best_choices);
    }

    println!("{}", board);
}

#[derive(Debug, Clone)]
struct Board {
    letters: [Letter; 5],
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Word([char; 5]);

#[derive(Debug, Clone)]
struct Letter {
    is: HashSet<char>,
}

impl Word {
    fn score(&self, chars: &[Vec<char>; 5]) -> usize {
        let mut score = 0;
        for (i, c) in chars.iter().enumerate() {
            let i = c
                .iter()
                .find_position(|x| **x == self.0[i])
                .map(|x| x.0)
                .unwrap_or(26);

            score += i;
        }
        score
    }
}

impl Board {
    fn word_is_ok(&self, word: Word) -> bool {
        for (i, c) in word.0.iter().enumerate() {
            if !self.letters[i].is.contains(c) {
                return false;
            }
        }
        true
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.0.iter() {
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            letters: Default::default(),
        }
    }
}

impl From<&str> for Word {
    fn from(value: &str) -> Self {
        let mut chars = [' '; 5];
        for (i, c) in value.chars().enumerate() {
            chars[i] = c;
        }
        Self(chars)
    }
}

impl Default for Letter {
    fn default() -> Self {
        Self {
            is: ('a'..='z').collect(),
        }
    }
}
impl Display for Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for c in self.is.iter() {
            s.push(*c);
        }
        write!(f, "{}", s)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for letter in self.letters.iter() {
            write!(f, "{}\n", letter)?;
        }
        Ok(())
    }
}
