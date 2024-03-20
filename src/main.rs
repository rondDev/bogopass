use std::{
    env,
    sync::{
        atomic::{AtomicI32, AtomicUsize, Ordering},
        Arc, Mutex,
    },
    thread,
    time::{self, Duration},
};

use rand::Rng;
fn main() {
    let start_time = time::Instant::now();
    let args: Vec<String> = env::args().collect();
    let letter_count;
    let symbol_count;
    let number_count;

    if args.len() > 3 && !args[1].is_empty() && !args[2].is_empty() && !args[3].is_empty() {
        letter_count = args[1].trim().parse::<usize>().unwrap();
        symbol_count = args[2].trim().parse::<usize>().unwrap();
        number_count = args[3].trim().parse::<usize>().unwrap();
    } else {
        println!("Welcome to bogopass!");

        println!("How many letters would you like in your password?");
        letter_count = read_num();

        println!("How many symbols would you like in your password?");
        symbol_count = read_num();

        println!("How many numbers would you like in your password?");
        number_count = read_num();
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

    let chars = Box::leak(char_set.into_boxed_str());
    let chars_len = chars.len() as usize;

    let num = Arc::new(AtomicI32::new(0));
    let dur_generate = Arc::new(AtomicUsize::new(0));
    let dur_check = Arc::new(AtomicUsize::new(0));

    let mut solved = false;

    let output_password = Arc::new(Mutex::new(String::from("")));
    let outp = output_password.clone();

    let mut temp: Vec<u8> = vec![0; letter_count + symbol_count + number_count];

    let n = Arc::clone(&num);
    let pass = Arc::clone(&output_password);
    let d_generate = Arc::clone(&dur_generate);
    let d_check = Arc::clone(&dur_check);

    // PERF: Threading allows for an increase in performance even if it's only a single thread
    // NOTE: Multiple threads seemed to worsen the performance in most of my benchmarks
    let thread = thread::spawn(move || loop {
        if solved {
            break;
        }
        let mut rng = rand::thread_rng();
        let before_generate = time::Instant::now();
        {
            for i in 0..(letter_count + symbol_count + number_count) {
                temp[i] = chars.chars().nth(rng.gen_range(0..chars_len)).unwrap() as u8;
            }
        }
        let duration_generate = before_generate.elapsed();
        d_generate.fetch_add(duration_generate.as_nanos() as usize, Ordering::SeqCst);
        n.fetch_add(1, Ordering::SeqCst);

        let before_check = time::Instant::now();
        if check_pass(&temp, letter_count, symbol_count, number_count) {
            solved = true;
            *pass.lock().unwrap() = String::from_utf8(temp.clone()).unwrap();
        }

        let duration_check = before_check.elapsed();
        d_check.fetch_add(duration_check.as_nanos() as usize, Ordering::SeqCst);
    });

    let _ = thread.join();
    let tries = num.load(Ordering::SeqCst);
    println!(
        "Your password is: {}\n\tIt took \t\t\t{:?} tries\n\tAverage time to generate: \t{:?}\n\tAverage time to check: \t\t{:?}",
        outp.lock().unwrap(),
        tries,
        (Duration::from_nanos(dur_generate.load(Ordering::SeqCst) as u64) / tries as u32),
        (Duration::from_nanos(dur_check.load(Ordering::SeqCst) as u64) / tries as u32),
    );
    println!("\tTotal time: \t\t\t{:?}", start_time.elapsed());
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
