use proc_const_regex::regex;


fn main() {
    callout("hello");
}

fn callout(to_test: &str) -> bool {
    regex!("123").test(to_test)
}