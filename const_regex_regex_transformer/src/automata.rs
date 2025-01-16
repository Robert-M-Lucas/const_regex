use std::fmt::{Debug, Formatter};
use std::ptr::write;
use const_regex_util::{char_to_utf8, next_char, CharSlice};
use crate::automata::TransitionType::{Any, ExcludeRange, Range, Single};
use crate::regex::{ChainedMatchable, Matchable, Repetition};

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
enum TransitionType {
    Single(u32),
    Range(u32, u32),
    ExcludeRange(u32, u32),
    Any,
}

pub struct DFA {
    pub transitions: Vec<Vec<(TransitionType, usize)>>
}

#[derive(Default, Clone)]
struct NFAState {
    transitions: Vec<(Option<TransitionType>, usize)>
}

impl NFAState {
    pub fn add_transition(&mut self, t: Option<TransitionType>, d: usize) {
        self.transitions.push((t, d))
    }
}


impl Debug for NFAState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (tt, ns) in &self.transitions {
            if let Some(tt) = tt {
                writeln!(f, "    {tt:?} -> {ns}")?;
            }
            else {
                writeln!(f, "    Empty -> {ns}")?;
            }
        }

        Ok(())
    }
}

pub struct NFA {
    pub states: Vec<NFAState>
}

impl Debug for NFA {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, s) in self.states.iter().enumerate() {
            writeln!(f, "{i}:")?;
            write!(f, "{s:?}")?;
        }

        Ok(())
    }
}

pub fn to_nfa(regex: ChainedMatchable) -> NFA {
    let mut states = Vec::new();
    let success_states = recursive_nfa(&regex, &mut states);

    // Make all success states succeed
    for s in success_states {
        let next = states.len();
        states[s].add_transition(None, next);
    }
    NFA { states }
}

fn recursive_nfa(section: &ChainedMatchable, transitions: &mut Vec<NFAState>) -> Vec<usize> {
    // Assume entry at state = transitions.len()
    // Make exit as transitions.len() after completion
    let (m, r, n) = (section.matchables(), section.repetition(), section.next());

    let start_state_pos = transitions.len();
    let start_state = NFAState::default();
    transitions.push(start_state);

    // Loop (repeat states)
    let (repeats, accepting, last_looped) = match r {
        Repetition::One => (1, 1..2, false),
        Repetition::Any => (1, 0..2, true),
        Repetition::AtLeast(x) => (*x, *x..*x+1, true),
        Repetition::LessThanEq(x) => (*x, 0..*x+1, false),
        Repetition::Range(x, y) => (*y, *x..*y+1, false),
    };

    let mut all_success_states = vec![start_state_pos];
    let mut prev_success_state = start_state_pos;

    for _ in 0..repeats {
        // Build all matchables
        let mut matchables_success_states = Vec::new();
        for m in m.matchable().matchables() {
            let t_len = transitions.len();
            transitions[prev_success_state].add_transition(None, t_len); // Transition to matchable
            let ss = matchable_nfa(m, transitions);
            matchables_success_states.extend_from_slice(&ss);
        }

        // Either tie all successes of matchables to success state or all failures
        // Based on whether inverted
        let success_state = NFAState::default();
        let success_state_pos = transitions.len();

        if *m.inverted() {
            for i in prev_success_state+1..transitions.len() {
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
        all_success_states.push(success_state_pos);
        prev_success_state = success_state_pos;
    }

    if last_looped {
        transitions[*all_success_states.last().unwrap()].add_transition(None, start_state_pos);
    }

    let final_success_state_pos = transitions.len();
    transitions.push(NFAState::default());


    for i in accepting {
        transitions[all_success_states[i as usize]].add_transition(None, final_success_state_pos);
    }

    let final_ends = if let Some(next) = n {
        let t_len = transitions.len();
        transitions[final_success_state_pos].add_transition(None, t_len);
        recursive_nfa(next, transitions)
    } else {
        vec![final_success_state_pos]
    };

    final_ends
}

fn matchable_nfa(matchable: &Matchable, transitions: &mut Vec<NFAState>) -> Vec<usize> {
    let t = match matchable {
        Matchable::Char(c) => {
            Some(Single(char_to_utf8(*c)))
        }
        Matchable::Range(a, b) => {
            Some(Range(char_to_utf8(*a), char_to_utf8(*b)))
        }
        Matchable::Any => Some(Any),
        Matchable::Subexpression(s) => {
            return recursive_nfa(s, transitions);
        }
    };

    let mut entry = NFAState::default();
    entry.add_transition(t, transitions.len() + 1);
    let success_pos = transitions.len() + 1;
    let success = NFAState::default();
    transitions.push(entry);
    transitions.push(success);

    vec![success_pos]
}

pub fn to_dfa(nfa: NFA) -> DFA {
    todo!()
}

pub fn test_nfa(input: &NFA, s: &str) -> bool {
    nfa_ant(input, 0, 0, s)
}

fn nfa_ant(nfa: &NFA, nfa_pos: usize, str_pos: usize, s: &str) -> bool {
    // println!("{}", nfa_pos);
    debug_assert!(nfa_pos <= nfa.states.len());
    if nfa_pos == nfa.states.len() {
        return str_pos == s.len();
    }
    let current_state = &nfa.states[nfa_pos];

    for (tt, ns) in &current_state.transitions {
        let ns = *ns;
        let r = if let Some(tt) = tt {
            if str_pos == s.bytes().len() {
                return false;
            }
            let (next_char, new_str_pos) = next_char(s, str_pos);
            debug_assert!(new_str_pos > str_pos);

            match tt {
                Single(c) => {
                    if *c == next_char { nfa_ant(nfa, ns, new_str_pos, s) }
                    else { false }
                }
                Range(a, b) => {
                    if next_char >= *a && next_char <= *b { nfa_ant(nfa, ns, new_str_pos, s) }
                    else { false }
                }
                ExcludeRange(a, b) => {
                    if *a >= next_char || *b <= next_char { nfa_ant(nfa, ns, new_str_pos, s) }
                    else { false }
                }
                Any => {
                    nfa_ant(nfa, ns, new_str_pos, s)
                }
            }
        }
        else {
            nfa_ant(nfa, ns, str_pos, s)
        };
        if r { return true }
    }
    false
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
                Any => true
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