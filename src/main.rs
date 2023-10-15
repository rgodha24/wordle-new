mod response;
mod word;

use clap::Parser;
use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use rayon::prelude::*;
use response::{Response, ResponseType};
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};
use word::{Letter, Word};

const GREEN_WEIGHT: usize = 100;
const YELLOW_WEIGHT: usize = 1;

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
    for run in 1.. {
        // let best_chars = best_chars(&answers);
        // let greens = board.greens();

        let best_choice = if run == 1 {
            "roate".into()
        } else {
            ok.par_iter()
                .progress_count(ok.len() as u64)
                .map(|w| (w, w.score_new(&answers)))
                .min_by_key(|x| x.1)
                .expect("best word exists")
                .0
                .clone()
        };

        println!("{}", best_choice);

        let response = Response::prompt_or_answer(&best_choice, answer.as_ref());

        if response.is_correct() {
            // println!("Found the word in {run} runs!");
            break;
        }

        board.use_responses(response, &best_choice);

        answers.retain(|w| board.word_is_ok(w.clone()));
        ok.retain(|w| board.word_is_ok(w.clone()));
    }
}

fn best_chars(answers: &HashSet<Word>) -> [Vec<char>; 5] {
    let mut answer_letters: [HashMap<char, usize>; 5] = Default::default();

    for answer in answers.iter() {
        for (i, c) in answer.iter().enumerate() {
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
    let mut ok: HashSet<Word> = ok.into_iter().map(|w| w.into()).collect();

    ok.extend(answers.iter().cloned());

    (answers, ok)
}

fn using_hashset<'a>(ok: &'a HashSet<Word>, answers: &'a HashSet<Word>) -> &'a HashSet<Word> {
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

impl Board {
    fn word_is_ok(&self, word: Word) -> bool {
        for (i, c) in word.iter().enumerate() {
            if !self.letters[i].contains(c) {
                return false;
            }
        }

        self.must_have.is_subset(&word.iter().copied().collect())
    }

    fn greens(&self) -> Vec<char> {
        self.letters
            .iter()
            .filter_map(|l| {
                if l.len() == 1 {
                    Some(*l.iter().next().unwrap())
                } else {
                    None
                }
            })
            .collect()
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

impl Default for Board {
    fn default() -> Self {
        Self {
            letters: Default::default(),
            must_have: Default::default(),
        }
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
