use std::{
    io::{BufRead, Write},
    ops::{Deref, Index},
};

use crate::Word;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Response([ResponseType; 5]);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ResponseType {
    Gray,
    Yellow,
    Green,
}

impl Response {
    pub fn is_correct(&self) -> bool {
        self.0.iter().all(|r| *r == ResponseType::Green)
    }

    fn prompt(word: &Word) -> Self {
        let mut r = [ResponseType::Gray; 5];
        for (i, c) in word.iter().enumerate() {
            r[i] = Self::prompt_single(c);
        }

        Self(r)
    }

    pub fn prompt_or_answer(guess: &Word, correct: Option<&Word>) -> Self {
        if let Some(correct) = correct {
            Self::from_answer(guess, correct)
        } else {
            Self::prompt(guess)
        }
    }

    fn prompt_single(c: &u8) -> ResponseType {
        print!("Is {} grey(1), yellow(2), or green(3)?: ", char::from(*c));
        std::io::stdout().flush().unwrap();

        let mut line = String::new();
        std::io::stdin().lock().read_line(&mut line).unwrap();

        match line.trim().parse::<usize>() {
            Ok(1) => ResponseType::Gray,
            Ok(2) => ResponseType::Yellow,
            Ok(3) => ResponseType::Green,
            Ok(n) => {
                println!("1, 2, or 3 only. You entered {}", n);
                Self::prompt_single(c)
            }
            Err(e) => {
                println!("1, 2, or 3 only. You entered {}", e);
                Self::prompt_single(c)
            }
        }
    }

    pub fn from_answer(guess: &Word, answer: &Word) -> Self {
        assert_eq!(answer.len(), 5);
        assert_eq!(guess.len(), 5);
        let mut c = [ResponseType::Gray; 5];
        // Array indexed by lowercase ascii letters
        let mut misplaced = [0u8; (b'z' - b'a' + 1) as usize];

        // Find all correct letters
        for ((&answer, &guess), c) in answer.iter().zip(guess.iter()).zip(c.iter_mut()) {
            if answer == guess {
                *c = ResponseType::Green
            } else {
                // If the letter does not match, count it as misplaced
                misplaced[(answer - b'a') as usize] += 1;
            }
        }
        // Check all of the non matching letters if they are misplaced
        for (&guess, c) in guess.iter().zip(c.iter_mut()) {
            // If the letter was guessed wrong and the same letter was counted as misplaced
            if *c == ResponseType::Gray && misplaced[(guess - b'a') as usize] > 0 {
                *c = ResponseType::Yellow;
                misplaced[(guess - b'a') as usize] -= 1;
            }
        }

        Response(c)
    }
}

impl ResponseType {
    pub fn is_misplaced(letter: u8, answer: &Word, used: &mut [bool; 5]) -> bool {
        answer.iter().enumerate().any(|(i, a)| {
            if *a == letter && !used[i] {
                used[i] = true;
                return true;
            }
            false
        })
    }
}

impl Index<usize> for Response {
    type Output = ResponseType;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Deref for Response {
    type Target = [ResponseType; 5];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
