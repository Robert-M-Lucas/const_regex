use const_regex_regex_transformer::automata::TransitionType;
use const_regex_regex_transformer::automata::TransitionType::{Any, ExcludeRange, Range, Single};
use const_regex_util::next_char;
use proc_const_regex::regex;



fn main() {
}

fn callout(to_test: &str) -> bool {
    regex!("123")(to_test)
}