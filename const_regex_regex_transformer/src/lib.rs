use crate::automata::{to_dfa, to_nfa, DFA};
use crate::regex::{parse_regex, ChainedMatchable};

mod automata;
pub mod regex;



pub fn to_regex(regex: &str) -> ChainedMatchable {
    let s = format!("({})", regex);
    parse_regex(&s, true).unwrap().1
}

pub fn to_automata(regex: ChainedMatchable) -> DFA {
    to_dfa(to_nfa(regex))
}
