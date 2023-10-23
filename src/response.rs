use std::{
    io::{BufRead, Write},
    ops::{Deref, Index},
};

use crate::Word;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Response([ResponseType; 5]);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ResponseType {
    Grey,
    Yellow,
    Green,
}

impl Response {
    pub fn is_correct(&self) -> bool {
        self.0.iter().all(|r| *r == ResponseType::Green)
    }

    fn prompt(word: &Word) -> Self {
        let mut r = [ResponseType::Grey; 5];
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

    pub fn from_answer(guess: &Word, correct: &Word) -> Self {
        let mut r = [ResponseType::Grey; 5];
        for (i, c) in guess.iter().enumerate() {
            if correct[i] == *c {
                r[i] = ResponseType::Green;
            } else if correct.contains(c) {
                r[i] = ResponseType::Yellow;
            }
        }

        Self(r)
    }

    fn prompt_single(c: &u8) -> ResponseType {
        print!("Is {} grey(1), yellow(2), or green(3)?: ", char::from(*c));
        std::io::stdout().flush().unwrap();

        let mut line = String::new();
        std::io::stdin().lock().read_line(&mut line).unwrap();

        match line.trim().parse::<usize>() {
            Ok(1) => ResponseType::Grey,
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

#[cfg(test)]
mod tests {
    use super::{Response, ResponseType};
    use crate::Word;

    #[test]
    fn from_answer_works() {
        use ResponseType::*;

        let guess = Word::from("hello");
        let ans = Word::from("world");
        let r = Response::from_answer(&guess, &ans);

        assert_eq!(r, Response([Grey, Grey, Yellow, Green, Yellow]));

        let guess = Word::from("saine");
        let correct = Word::from("agent");

        let r = Response::from_answer(&guess, &correct);
        assert_eq!(r, Response([Grey, Yellow, Grey, Green, Yellow]));
    }
}
