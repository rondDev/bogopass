use std::{
    sync::{
        atomic::{AtomicI32, Ordering},
        Arc, Mutex,
    },
    thread,
};

use rand::Rng;
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

    let num = Arc::new(AtomicI32::new(0));

    let mut solved = false;

    let output_password = Arc::new(Mutex::new(String::from("")));
    let outp = output_password.clone();

    let n = Arc::clone(&num);
    let pass = Arc::clone(&output_password);
    let thread = thread::spawn(move || loop {
        if solved {
            break;
        }
        let mut rng = rand::thread_rng();
        let mut o = String::from("");
        for _ in 0..(letter_count + symbol_count + number_count) {
            o.push(
                chars
                    .chars()
                    .nth(rng.gen_range(0..chars.len()) as usize)
                    .unwrap(),
            );
        }
        n.fetch_add(1, Ordering::SeqCst);
        if check_pass(&o, letter_count, symbol_count, number_count) {
            solved = true;
            *pass.lock().unwrap() = o.to_string();
        }
    });

    let _ = thread.join();
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

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
