use ascii::AsciiStr;
use itertools::Itertools;
use nom::character::complete::{anychar, char};
use nom::IResult;

type NResult<'a, T> = IResult<&'a str, T>;


struct ChainedMatchable {
    matchables: InvertibleMatchable,
    repetition: Repetition,
    next: Option<Box<ChainedMatchable>>
}

enum Repetition {
    None,
    Any,
    RangeMin(usize),
    RangeMax(usize),
    Range(usize, usize)
}

struct InvertibleMatchable {
    inverted: bool,
    matchable: UnionMatchables
}

struct UnionMatchables {
    matchables: Vec<Matchable>
}

enum Matchable {
    Char(char),
    Start,
    End,
    Range([u8; 4], [u8; 4]),
    Any,
    Subexpression(ChainedMatchable)
}

impl Matchable {
    pub fn is_quantifiable(&self) -> bool {
        match &self {
            Matchable::Char(_) => { true }
            Matchable::Start => { false }
            Matchable::End => { false }
            Matchable::Range(_, _) => { true }
            Matchable::Any => { true }
            Matchable::Subexpression(_) => { true }
        }
    }
}

pub fn to_regex(regex: &str) -> ChainedMatchable {
    let s = format!("({})", regex);
    parse_regex(&s).unwrap().1
}

fn parse_regex(regex: &str) -> NResult<ChainedMatchable> {
    let (r, _) = char('(')(regex)?;

    let mut ors = Vec::new();

    let mut rm = r;
    loop {
        let (r, base_char) = anychar(rm)?;
        rm = r;
        let base = match base_char {
            '\\' => {
                todo!()
            }
            '[' => {
                todo!()
            }
            '(' => {
                todo!()
            }
            '{' => {
                panic!()
            }
            '}' => {
                panic!()
            }
            c => {
                InvertibleMatchable {
                    inverted: false,
                    matchable: UnionMatchables {
                        matchables: vec![match c {
                            '^' => Matchable::Start,
                            '$' => Matchable::End,
                            '.' => Matchable::Any,
                            c => Matchable::Char(c)
                        }],
                    },
                }
            }
        };

        let repetition = if let Some((r, _)) = char('*')(rm) {
            rm = r;
            Repetition::Any
        } else if let Some((r, _)) = char('{')(rm) {
            rm = r;
            todo!()
        }
        else {
            Repetition::None
        };

        ors.push(ChainedMatchable {
            matchables: base,
            repetition,
            next: None,
        });

        if let Ok((r, _)) = char('|')(r) {
            rm = r;
            continue;
        }
        break;
    }

    debug_assert!(!ors.is_empty());
    let mut m = if ors.len() == 1 {
        ors.into_iter().next().unwrap()
    }
    else {
        ChainedMatchable {
            matchables: InvertibleMatchable {
                inverted: false,
                matchable: UnionMatchables {
                    matchables: ors.into_iter().map(|o| {
                        Matchable::Subexpression(o)
                    }).collect_vec(),
                }
            },
            repetition: Repetition::None,
            next: None,
        }
    };


    if let Ok((r, _)) = char(')')(r) {
        return Ok((r, m))
    }

    let (r, c) = parse_regex(r)?;
    m.next = Some(Box::new(c));
    Ok((r, m))
}

