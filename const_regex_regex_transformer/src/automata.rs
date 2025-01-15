use const_regex_util::{char_to_utf8, next_char};
use crate::automata::TransitionType::{ExcludeRange, Range, Single};
use crate::regex::{ChainedMatchable, Matchable};

#[repr(u8)]
enum TransitionType {
    Single(u32),
    Range(u32, u32),
    ExcludeRange(u32, u32),
}

pub struct DFA {
    pub transitions: Vec<Vec<(TransitionType, usize)>>
}

#[derive(Default)]
struct NFAState {
    transitions: Vec<(Option<TransitionType>, usize)>
}

impl NFAState {
    pub fn add_transition(&mut self, t: Option<TransitionType>, d: usize) {
        self.transitions.push((t, d))
    }
}

struct NFA {
    pub states: Vec<NFAState>
}

pub fn to_nfa(regex: ChainedMatchable) -> NFA {
    let mut states = Vec::new();
    let success_states = recursive_nfa(&regex, &mut states);

    // Make all success states succeed
    for s in success_states {
        states[s].add_transition(None, states.len() + 1);
    }
    NFA { states }
}

pub fn recursive_nfa(section: &ChainedMatchable, transitions: &mut Vec<NFAState>) -> Vec<usize> {
    // Assume entry at state = transitions.len()
    // Make exit as transitions.len() after completion
    let (m, r, n) = (section.matchables(), section.repetition(), section.next());

    // Start state with empty transitions to all matchables
    let mut start_state = NFAState::default();
    let start_state_pos = transitions.len();
    for i in 0..m.matchable().matchables().len() {
        start_state.add_transition(None, start_state_pos + i);
    }
    transitions.push(start_state);

    // Build all matchables
    let mut matchables_success_states = Vec::new();
    for m in m.matchable().matchables() {
        let ss = matchable_nfa(m, transitions);
        matchables_success_states.extend_from_slice(&[ss]);
    }

    // Either tie all successes of matchables to success state or all failures

    // Loop (repeat states)

    todo!();
}

pub fn matchable_nfa(matchable: &Matchable, transitions: &mut Vec<NFAState>) -> Vec<usize> {
    todo!()
}

pub fn to_dfa(nfa: NFA) -> DFA {
    todo!()
}

const fn test(input: &str) -> bool {
    const T1: [(TransitionType, usize); 2] = [(Single(char_to_utf8('a')), 0), (Range(char_to_utf8('d'), char_to_utf8('f')), 1)];

    const TRANSITIONS: [&[(TransitionType, usize)]; 1] = [&T1];

    let mut s = 0;
    loop {
        if s >= TRANSITIONS.len() {
            return true;
        }
        let ts = &TRANSITIONS[s];
        let (c, d) = next_char(input, s);
        s += d;

        let mut i = 0;
        let len = ts.len();
        while i < len {
            let (t, ns) = &ts[i];

            let r = match t {
                Single(a) => *a == c,
                Range(a, b) => *a <= c && c <= *b,
                ExcludeRange(a, b) => c < *a || *b > c,
            };

            if r {
                s = *ns;
                continue;
            }

            i += 1;
        }

        return false
    }
}