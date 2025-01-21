use proc_const_regex::regex;


fn main() {
    println!("{}", callout("hello"));
}

const fn callout(to_test: &str) -> bool {
    regex!("hello").test(to_test)
}