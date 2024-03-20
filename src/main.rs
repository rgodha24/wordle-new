mod guess;
mod response;
mod word;

use clap::Parser;
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use itertools::Itertools;
use rayon::prelude::*;
use response::Response;
use std::{cell::OnceCell, collections::HashSet, process};
use word::Word;

use crate::guess::{CachedGuess, Guess};

#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(short, long)]
    answer: Option<String>,

    /// Generate the cbor file of all possible matches
    /// NOTE: THIS FILE TAKES LIKE 10 MINS TO MAKE AND IS 253 MB
    #[arg(short, long, default_value_t = false)]
    gen_cbor: bool,

    /// Use the cbor file of all possible matches
    /// NOTE: this is slower
    #[arg(short, long, default_value_t = false)]
    use_cached_matches: bool,

    /// calculate the best word to guess instead of using `slate`
    #[arg(short, long, default_value_t = false)]
    use_best_word: bool,

    #[arg(long, default_value=None)]
    starting_word: Option<String>,
}

const MATCHES: OnceCell<HashSet<u64>> = OnceCell::new();

fn main() {
    let args = Args::parse();
    let answer: Option<Word> = args.answer.map(|s| s.as_str().into());

    if args.gen_cbor {
        generate_matches();
        return;
    }

    let (mut answers, mut ok) = read_jsons();
    if args.use_cached_matches {
        read_matches();
    }

    let starting_word: Word = match args.starting_word {
        Some(s) if s.len() == 5 => s.as_str().into(),
        Some(_) => {
            eprintln!("starting word must be 5 chars long");
            "saine".into()
        }
        None => "saine".into(),
    };

    // not 0 index...
    for run in 1..10 {
        eprintln!("{} ok {} answers", ok.len(), answers.len());

        // let best_chars = best_chars(&answers);
        // let greens = board.greens();

        let best_choice = if run == 1 && !args.use_best_word {
            starting_word.clone()
        } else if answers.len() < 2 {
            answers.iter().next().unwrap().clone()
        } else {
            ok.par_iter()
                .progress_count(ok.len() as u64)
                .map(|w| (w, w.score_new(&answers, &run)))
                .min_by_key(|x| x.1)
                .expect("no words left")
                .0
                .clone()
        };

        println!("{}", best_choice);

        let response = Response::prompt_or_answer(&best_choice, answer.as_ref());

        if response.is_correct() {
            eprintln!("Found the word in {run} runs!");
            process::exit(0);
        }

        let guess = Guess {
            word: best_choice,
            mask: response,
        };

        answers.retain(|w| guess.matches_cached(w));
        ok.retain(|w| guess.matches_cached(w));
    }

    panic!("damn i suck at coding");
}

fn read_jsons() -> (HashSet<Word>, HashSet<Word>) {
    let ok = include_str!("../ok.json");
    let answers = include_str!("../answers.json");
    let ok: HashSet<&str> = serde_json::from_str(ok).unwrap();
    let answers: HashSet<&str> = serde_json::from_str(answers).unwrap();

    let answers: HashSet<Word> = answers.into_iter().map(|w| w.into()).collect();
    let mut ok: HashSet<Word> = ok.into_iter().map(|w| w.into()).collect();

    ok.extend(answers.iter().cloned());

    (answers, ok)
}

fn read_matches() {
    let file = std::fs::File::open("matches.cbor").unwrap();

    let pb = ProgressBar::new(file.metadata().unwrap().len());

    pb.set_style(
        ProgressStyle::with_template("{wide_bar} {bytes}/{total_bytes} {eta} {elapsed}").unwrap(),
    );

    MATCHES
        .set(ciborium::from_reader(pb.wrap_read(file)).unwrap())
        .unwrap();
}

#[allow(dead_code)]
fn generate_matches() {
    let (answers, ok) = read_jsons();

    let matches: Vec<u64> = answers
        .iter()
        .cartesian_product((0..125).map(Response::from))
        .par_bridge()
        .progress_count(answers.len() as u64 * 125)
        .map(|(a, m)| {
            let mut nums = vec![];
            let guess = Guess {
                word: a.clone(),
                mask: m,
            };
            for o in &ok {
                if guess.matches(o) {
                    nums.push(CachedGuess::from((&guess, o)).0);
                }
            }

            nums
        })
        .flatten()
        .collect();

    println!("matches: {}", matches.len());

    let out_file = std::fs::File::create("matches.cbor").unwrap();

    ciborium::into_writer(&matches, out_file).unwrap();
}
