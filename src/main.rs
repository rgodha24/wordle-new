use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    io::{BufRead, Write},
};

fn main() {
    let ok = include_str!("../ok.json");
    let answers = include_str!("../answers.json");
    let ok: HashSet<&str> = serde_json::from_str(&ok).unwrap();
    let answers: HashSet<&str> = serde_json::from_str(&answers).unwrap();

    let mut answers: HashSet<Word> = answers.into_iter().map(|w| w.into()).collect();
    let mut ok: HashSet<Word> = ok.into_iter().map(|w| w.into()).collect();
    let mut board = Board::default();

    // not 0 index...
    for run in 1..=6 {
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

        let best_choice = using_hashset(&ok, &answers)
            .par_iter()
            .progress_count(ok.len() as u64)
            .map(|w| (w, w.score(&best_chars)))
            .min_by_key(|x| x.1)
            .expect("best word exists")
            .0;

        println!("{}", best_choice);

        let responses = best_choice.get_responses();

        if responses.iter().all(|r| *r == Response::Green) {
            println!("Found the word in {run} runs!");
            break;
        }

        board.use_responses(responses, best_choice);

        answers.retain(|w| board.word_is_ok(w.clone()));
        ok.retain(|w| board.word_is_ok(w.clone()));
    }
}

fn using_hashset<'a>(ok: &'a HashSet<Word>, answers: &'a HashSet<Word>) -> &'a HashSet<Word> {
    println!("ok: {}, answers: {}", ok.len(), answers.len());
    if answers.len() > 5 {
        ok
    } else {
        answers
    }
}

fn prompt_response(c: char) -> Response {
    print!("Is {c} grey(1), yellow(2), or green(3)?: ");
    std::io::stdout().flush().unwrap();

    let mut line = String::new();
    std::io::stdin().lock().read_line(&mut line).unwrap();

    match line.trim().parse::<usize>() {
        Ok(1) => Response::Grey,
        Ok(2) => Response::Yellow,
        Ok(3) => Response::Green,
        Ok(n) => {
            println!("1, 2, or 3 only. You entered {}", n);
            prompt_response(c)
        }
        Err(e) => {
            println!("1, 2, or 3 only. You entered {}", e);
            prompt_response(c)
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Response {
    Grey,
    Yellow,
    Green,
}

#[derive(Debug, Clone)]
struct Board {
    letters: [Letter; 5],
    must_have: HashSet<char>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Word([char; 5]);

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

    fn get_responses(&self) -> [Response; 5] {
        let mut responses = [Response::Grey; 5];
        for (i, c) in self.0.iter().enumerate() {
            responses[i] = prompt_response(*c);
        }
        responses
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

    fn use_responses(&mut self, responses: [Response; 5], word: &Word) {
        for (i, r) in responses.iter().enumerate() {
            match r {
                Response::Grey => {
                    self.letters[i].remove_choice(word.0[i]);
                }
                Response::Yellow => {
                    self.letters[i].remove_choice(word.0[i]);
                    self.must_have.insert(word.0[i]);
                }
                Response::Green => {
                    self.letters[i].set_choice(word.0[i]);
                }
            }
        }
    }

    fn remove_from_all(&mut self, c: char) {
        self.letters.iter_mut().for_each(|l| {
            l.remove_choice(c);
        });
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
