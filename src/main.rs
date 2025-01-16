use std::collections::HashSet;
use const_regex_regex_transformer::automata::{test_nfa, to_dfa, to_nfa};
use const_regex_regex_transformer::to_regex;
use const_regex_util::next_char;

fn main() {
    let r = to_regex("123[abc]");

    println!("{:?}", r);

    let nfa = to_nfa(r);

    let dfa = to_dfa(nfa);

    println!("{:?}", dfa);

    // println!("{:?}", nfa);
    //
    // println!("{}", test_nfa(&nfa, "13"))
}