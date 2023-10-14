mod response;

use clap::Parser;
use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use rayon::prelude::*;
use response::{Response, ResponseType};
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    ops::{Deref, Index},
};

#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(short, long)]
    answer: Option<String>,
}

fn main() {
    let args = Args::parse();
    let answer: Option<Word> = args.answer.map(|s| s.as_str().into());
    let mut board = Board::default();

    let (mut answers, mut ok) = read_jsons();

    // not 0 index...
    for run in 1..=6 {
        let best_chars = best_chars(&answers);

        let best_choice = using_hashset(&ok, &answers)
            .par_iter()
            .map(|w| (w, w.score(&best_chars)))
            .min_by_key(|x| x.1)
            .expect("best word exists")
            .0;

        println!("{}", best_choice);

        let response = Response::prompt_or_answer(best_choice, answer.as_ref());

        if response.is_correct() {
            // println!("Found the word in {run} runs!");
            std::process::exit(0);
        }

        board.use_responses(response, best_choice);

        answers.retain(|w| board.word_is_ok(w.clone()));
        ok.retain(|w| board.word_is_ok(w.clone()));
    }

    println!("Couldn't find the word in 6 runs :(");
}

fn best_chars(answers: &HashSet<Word>) -> [Vec<char>; 5] {
    let mut answer_letters: [HashMap<char, usize>; 5] = Default::default();

    for answer in answers.iter() {
        for (i, c) in answer.0.iter().enumerate() {
            *answer_letters[i].entry(*c).or_default() += 1;
        }
    }

    answer_letters
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
        .unwrap()
}

fn read_jsons() -> (HashSet<Word>, HashSet<Word>) {
    let ok = include_str!("../ok.json");
    let answers = include_str!("../answers.json");
    let ok: HashSet<&str> = serde_json::from_str(&ok).unwrap();
    let answers: HashSet<&str> = serde_json::from_str(&answers).unwrap();

    let answers: HashSet<Word> = answers.into_iter().map(|w| w.into()).collect();
    let ok: HashSet<Word> = ok.into_iter().map(|w| w.into()).collect();

    (answers, ok)
}

fn using_hashset<'a>(ok: &'a HashSet<Word>, answers: &'a HashSet<Word>) -> &'a HashSet<Word> {
    // println!("ok: {}, answerinput: {}", ok.len(), answers.len());
    if answers.len() > 5 {
        ok
    } else {
        answers
    }
}

#[derive(Debug, Clone)]
struct Board {
    letters: [Letter; 5],
    must_have: HashSet<char>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Word([char; 5]);

#[derive(Debug, Clone)]
struct Letter {
    is: HashSet<char>,
}

impl Word {
    fn score(&self, best_chars: &[Vec<char>; 5]) -> usize {
        let mut score = 0;
        for (i, chars) in best_chars.iter().enumerate() {
            let i = chars
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

        self.must_have.is_subset(&word.0.iter().copied().collect())
    }

    fn use_responses(&mut self, responses: Response, word: &Word) {
        for (i, r) in responses.iter().enumerate() {
            match r {
                ResponseType::Grey => {
                    self.letters[i].remove_choice(word[i]);
                }
                ResponseType::Yellow => {
                    self.letters[i].remove_choice(word[i]);
                    self.must_have.insert(word[i]);
                }
                ResponseType::Green => {
                    self.letters[i].set_choice(word[i]);
                }
            }
        }
    }
}

impl Letter {
    fn remove_choice(&mut self, c: char) -> bool {
        self.is.remove(&c)
    }
    fn set_choice(&mut self, c: char) {
        self.is = [c].into_iter().collect();
    }
}

impl Index<usize> for Word {
    type Output = char;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Deref for Word {
    type Target = [char; 5];

    fn deref(&self) -> &Self::Target {
        &self.0
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
            must_have: Default::default(),
        }
    }
}

impl From<&str> for Word {
    fn from(value: &str) -> Self {
        if value.len() != 5 {
            panic!("Word {value} isn't 5 characters long");
        }

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
