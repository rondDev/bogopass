use std::{
    env,
    sync::{Arc, Mutex},
    thread,
    time::{self, Duration},
};

use num_format::{Locale, ToFormattedString};
use rand::{thread_rng, Rng};
fn main() {
    let start_time = time::Instant::now();
    let args: Vec<String> = env::args().collect();
    let letter_count;
    let symbol_count;
    let number_count;
    let thread_count;

    if args.len() > 3
        && !args[1].is_empty()
        && !args[2].is_empty()
        && !args[3].is_empty()
        && !args[4].is_empty()
    {
        letter_count = args[1].trim().parse::<usize>().unwrap();
        symbol_count = args[2].trim().parse::<usize>().unwrap();
        number_count = args[3].trim().parse::<usize>().unwrap();
        thread_count = args[4].trim().parse::<usize>().unwrap();
    } else {
        println!("Welcome to bogopass!");

        println!("How many letters would you like in your password?");
        letter_count = read_num();

        println!("How many symbols would you like in your password?");
        symbol_count = read_num();

        println!("How many numbers would you like in your password?");
        number_count = read_num();

        println!("Amount of threads?");
        thread_count = read_num();
    }

    let ascii_letters = String::from_utf8((b'a'..=b'z').chain(b'A'..=b'Z').collect()).unwrap();
    let punctuation = String::from_utf8(
        (b'!'..=b'/')
            .chain(b':'..=b'@')
            .chain(b'{'..=b'~')
            .collect(),
    )
    .unwrap();
    let digits = String::from_utf8((b'0'..=b'9').collect()).unwrap();

    let mut char_set: String = String::new();

    if letter_count > 0 {
        char_set += ascii_letters.as_str();
    }

    if symbol_count > 0 {
        char_set += punctuation.as_str();
    }

    if number_count > 0 {
        char_set += digits.as_str();
    }

    let solved = Arc::new(Mutex::new(false));
    let total = Arc::new(Mutex::new(0));
    let pass = Arc::new(Mutex::new(String::new()));

    thread::scope(|s| {
        let mut threads = vec![];
        for _ in 0..thread_count {
            threads.push(s.spawn(|| loop {
                if *solved.lock().unwrap() {
                    break;
                }
                *total.lock().unwrap() += 1;
                let p = new_impl(&letter_count, &symbol_count, &number_count, &char_set);
                let c = check_pass(
                    p.as_str().as_bytes(),
                    letter_count,
                    symbol_count,
                    number_count,
                );
                if c {
                    *solved.lock().unwrap() = true;
                    *pass.lock().unwrap() = p.clone();
                    break;
                }
            }))
        }
    });

    let total = *total.lock().unwrap();
    println!(
        "Letters: {}\nSymbols: {}\nNumbers: {}\nThreads: {}",
        letter_count, symbol_count, number_count, thread_count
    );
    println!(
        "Your password is: {}\n\tIterations:\t\t\t{}\n\tAverage time per iteration:\t{:?}\n\tTotal time: \t\t\t{:?}",
        *pass.lock().unwrap(),
        total.to_formatted_string(&Locale::en),
        Duration::from_nanos(start_time.elapsed().as_nanos() as u64 / total),
        start_time.elapsed()
    );
}

fn check_pass(pass: &[u8], letters: usize, symbols: usize, numbers: usize) -> bool {
    let mut letter_count = 0;
    let mut symbol_count = 0;
    let mut number_count = 0;
    for c in pass {
        if c.is_ascii_alphabetic() {
            letter_count += 1;
            continue;
        }
        if c.is_ascii_punctuation() {
            symbol_count += 1;
            continue;
        }
        if c.is_ascii_digit() {
            number_count += 1;
            continue;
        }
    }
    letters == letter_count && symbols == symbol_count && numbers == number_count
}

fn read_string() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("can not read user input");
    input
}

fn read_num() -> usize {
    read_string().trim().parse::<usize>().unwrap()
}

fn gen_string(len: usize, charset: String) -> String {
    let mut rng = thread_rng();
    let s: String = (0..len)
        .map(|_| {
            (charset
                .chars()
                .nth(rng.gen_range(0..charset.len()))
                .unwrap()) as char
        })
        .collect();
    s
}

fn new_impl(
    letter_len: &usize,
    symbol_len: &usize,
    number_len: &usize,
    charset: &String,
) -> String {
    let s = gen_string(letter_len + symbol_len + number_len, charset.to_string());
    s
}
