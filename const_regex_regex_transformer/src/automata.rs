use const_regex_util::{char_to_utf8, next_char};
use crate::automata::TransitionType::{ExcludeRange, Range, Single};
use crate::regex::{ChainedMatchable, Matchable, Repetition};

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

pub struct NFA {
    pub states: Vec<NFAState>
}

pub fn to_nfa(regex: ChainedMatchable) -> NFA {
    let mut states = Vec::new();
    let success_states = recursive_nfa(&regex, &mut states);

    // Make all success states succeed
    for s in success_states {
        let next = states.len() + 1;
        states[s].add_transition(None, next);
    }
    NFA { states }
}

pub fn recursive_nfa(section: &ChainedMatchable, transitions: &mut Vec<NFAState>) -> Vec<usize> {
    // Assume entry at state = transitions.len()
    // Make exit as transitions.len() after completion
    let (m, r, n) = (section.matchables(), section.repetition(), section.next());

    let start_state_pos = transitions.len();
    let start_state = NFAState::default();
    transitions.push(start_state);

    // Build all matchables
    let mut matchables_success_states = Vec::new();
    for m in m.matchable().matchables() {
        transitions[start_state_pos].add_transition(None, transitions.len()); // Transition to matchable
        let ss = matchable_nfa(m, transitions);
        matchables_success_states.extend_from_slice(&ss);
    }

    // Either tie all successes of matchables to success state or all failures
    let success_state = NFAState::default();
    let success_state_pos = transitions.len();

    if *m.inverted() {
        for i in start_state_pos+1..transitions.len() {
            if matchables_success_states.contains(&i) {
                continue;
            }
            transitions[i].add_transition(None, success_state_pos);
        }
    }
    else {
        for i in matchables_success_states {
            transitions[i].add_transition(None, success_state_pos);
        }
    }

    transitions.push(success_state);

    // Loop (repeat states)
    let (repeats, accepting, last_looped) = match r {
        Repetition::One => (1, 0..1, false),
        Repetition::Any => (1, 0..1, true),
        Repetition::AtLeast(x) => (*x, *x-1..*x, true),
        Repetition::LessThanEq(x) => (*x, 0..*x, false),
        Repetition::Range(x, y) => (*y, *x..*y, false),
    };

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