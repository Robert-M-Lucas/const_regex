use proc_const_regex::regex;
use regex::Regex;
use std::hint::black_box;
use std::time::Instant;


fn main() {
    assert!(my_implementation("helloo"));
    assert!(my_implementation("hellxo"));
    assert!(my_implementation("hellxxo"));
    assert!(!my_implementation("hellxx"));
    assert!(!my_implementation("hell1o"));

    const TOTAL_RUNS: usize = 1_000_000;

    const TEST_SUIT: [&'static str; 4] = [
        "Hello",
        "hello",
        "hellooo",
        "none"
    ];

    let start = Instant::now();
    for x in 0..TOTAL_RUNS {
        let i = x % TEST_SUIT.len();
        black_box(
            my_implementation(black_box(TEST_SUIT[i]))
        );
    }
    let my_time = start.elapsed();

    println!("Mine            : {:?} | {:?} per request", my_time, my_time / TOTAL_RUNS as u32);

    let start = Instant::now();
    for x in 0..TOTAL_RUNS {
        let i = x % TEST_SUIT.len();
        black_box(
            regex_naive_implementation(black_box(TEST_SUIT[i]))
        );
    }
    let regex_naive_time = start.elapsed();

    println!("Regex [naive   ]: {:?} | {:?} per request", regex_naive_time, regex_naive_time / TOTAL_RUNS as u32);

    let re = Regex::new("^hell[a-z]+o+$").unwrap();
    let start = Instant::now();
    for x in 0..TOTAL_RUNS {
        let i = x % TEST_SUIT.len();
        black_box(
            regex_sensible_implementation(&re, black_box(TEST_SUIT[i]))
        );
    }
    let regex_sensible_time = start.elapsed();

    println!("Regex [sensible]: {:?} | {:?} per request", regex_sensible_time, regex_sensible_time / TOTAL_RUNS as u32);
}


const fn my_implementation(to_test: &str) -> bool {
    regex!("hell[a-z]+o+").test(to_test)
}

fn regex_naive_implementation(to_test: &str) -> bool {
    let re = Regex::new("^hell[a-z]+o+$").unwrap();
    re.is_match(to_test)
}

fn regex_sensible_implementation(re: &Regex, to_test: &str) -> bool {
    re.is_match(to_test)
}