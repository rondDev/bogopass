use std::{
    sync::{
        atomic::{AtomicI32, Ordering},
        Arc, Mutex,
    },
    thread,
};

use randomizer::Randomizer;

fn main() {
    let ascii_letters = String::from_utf8((b'a'..=b'z').chain(b'A'..=b'Z').collect()).unwrap();
    let punctuation = String::from_utf8(
        (b'!'..=b'/')
            .chain(b':'..=b'@')
            .chain(b'{'..=b'~')
            .collect(),
    )
    .unwrap();
    let digits = String::from_utf8((b'0'..=b'9').collect()).unwrap();

    println!("Welcome to bogopass!");

    println!("How many letters would you like in your password?");
    let letter_count = read_string().trim().parse::<usize>().unwrap();

    println!("How many symbols would you like in your password?");
    let symbol_count = read_string().trim().parse::<usize>().unwrap();

    println!("How many numbers would you like in your password?");
    let number_count = read_string().trim().parse::<usize>().unwrap();

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

    let chars = string_to_static_str(char_set);

    println!("Amount of threads?");
    let thread_count = read_string().trim().parse::<i32>().unwrap();

    let num = Arc::new(AtomicI32::new(0));

    let mut solved = false;

    let mut threads = vec![];

    let output_password = Arc::new(Mutex::new(String::from("")));
    let outp = output_password.clone();

    for _ in 0..thread_count {
        let n = Arc::clone(&num);
        let pass = Arc::clone(&output_password);
        threads.push(thread::spawn(move || loop {
            if solved {
                break;
            }
            let p = Randomizer::new(
                letter_count.clone() + symbol_count + number_count,
                Some(chars),
            )
            .string()
            .unwrap();
            n.fetch_add(1, Ordering::SeqCst);
            if check_pass(&p, letter_count, symbol_count, number_count) {
                solved = true;
                *pass.lock().unwrap() = p.to_string();
            }
            // println!("{} on try {}", p, n.load(Ordering::SeqCst));
        }));
    }

    for thread in threads {
        let _ = thread.join();
    }
    println!(
        "Your password is: {} and it took {:?} tries.",
        outp.lock().unwrap(),
        num.load(Ordering::SeqCst)
    );
}

fn check_pass(pass: &str, letters: usize, symbols: usize, numbers: usize) -> bool {
    let mut letter_count = 0;
    let mut symbol_count = 0;
    let mut number_count = 0;
    for k in pass.chars() {
        let c = k as u8;
        if (c >= 65 && c <= 90) || (c >= 97 && c <= 122) {
            letter_count += 1;
            continue;
        }
        if (c >= 33 && c <= 47)
            || (c >= 58 && c <= 64)
            || (c >= 91 && c <= 96)
            || (c >= 123 && c <= 126)
        {
            symbol_count += 1;
            continue;
        }
        if c >= 48 && c <= 57 {
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

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
