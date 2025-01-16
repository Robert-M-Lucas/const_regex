use const_regex_regex_transformer::automata::{test_nfa, to_nfa};
use const_regex_regex_transformer::to_regex;
use const_regex_util::next_char;

fn main() {
    let r = to_regex("12[a-d]");

    println!("{:#?}", r);

    let nfa = to_nfa(r);

    println!("{:?}", nfa);


    println!("{}", test_nfa(&nfa, "123"))
}