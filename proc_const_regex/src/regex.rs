use std::collections::VecDeque;
use either::{Either, Right, Left};
use itertools::Itertools;
use nom::character::complete::{anychar, digit1};
use nom::character::complete::char as cchar;
use nom::combinator::{map_res, opt, recognize};
use nom::error::{Error, ParseError};
use nom::IResult;

type NResult<'a, T> = IResult<&'a str, T>;


pub struct ChainedMatchable {
    matchables: InvertibleMatchable,
    repetition: Repetition,
    next: Option<Box<ChainedMatchable>>
}

enum Repetition {
    None,
    Any,
    AtLeast(u64),
    LessThanEq(u64),
    Range(u64, u64)
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
    Range(char, char),
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

fn parse_match_group(regex: &str) -> NResult<InvertibleMatchable> {
    let (r, _) = cchar::<_, Error<_>>('[')(regex)?;

    let (r, inverted) = if let Ok((r, _)) = cchar::<&str, Error<_>>('^')(r) {
        (r, true)
    } else {
        (r, false)
    };


    let mut rm = r;
    let mut chars: VecDeque<Either<char, _>> = VecDeque::new();
    loop {
        let (r, base_char) = anychar(rm)?;
        rm = r;

        match base_char {
            '\\' => {
                todo!()
            }
            ']' => {
                break;
            }
            '-' => {
                chars.push_back(Right(()));
            }
            c => {
                chars.push_back(Left(c));
            }
        };
    }

    if chars.is_empty() {
        panic!()
    }

    let mut ms = vec![
        Matchable::Char(match chars.pop_front().unwrap() {
            Left(c) => {c}
            Right(_) => {'-'}
        })
    ];

    while chars.len() > 1 {
        if chars[1].is_right() {
            let start = chars.pop_front().unwrap().unwrap_left();
            chars.pop_front().unwrap().unwrap_right();
            let end = chars.pop_front();
            if let Some(end) = end {
                ms.push(Matchable::Range(start, end.unwrap_left()))
            }
            else {
                ms.push(Matchable::Char(start));
                ms.push(Matchable::Char('-')); // ended with -
            }
        }
        else {
            ms.push(Matchable::Char(chars.pop_front().unwrap().unwrap_left()))
        }
    }

    if !chars.is_empty() {
        ms.push(Matchable::Char(match chars.pop_front().unwrap() {
            Left(c) => {c}
            Right(_) => {'-'}
        }));
    }

    Ok((rm, InvertibleMatchable {
        inverted,
        matchable: UnionMatchables {
            matchables: ms
        },
    }))
}

fn parse_u64(input : &str) -> NResult<u64> {
    map_res(recognize(digit1), str::parse)(input)
}


fn parse_regex(regex: &str) -> NResult<ChainedMatchable> {
    let (r, _) = cchar::<_, Error<_>>('(')(regex)?;

    let mut ors = Vec::new();

    let mut rm = r;
    loop {
        let (r, base_char) = anychar(rm)?;

        let base = match base_char {
            '\\' => {
                rm = r;
                todo!()
            }
            '[' => {
                let (r, i) = parse_match_group(rm)?;
                rm = r;
                i
            }
            '(' => {
                let (r, c) = parse_regex(rm)?;
                rm = r;
                InvertibleMatchable {
                    inverted: false,
                    matchable: UnionMatchables {
                        matchables: vec![Matchable::Subexpression(c)],
                    },
                }
            }
            '{' => {
                panic!()
            }
            '}' => {
                panic!()
            }
            c => {
                rm = r;
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

        let repetition = if let Ok((r, _)) = cchar::<_, Error<_>>('*')(rm) {
            rm = r;
            Repetition::Any
        } else if let Ok((r, _)) = cchar::<_, Error<_>>('{')(rm) {
            rm = r;
            let (r, min) = opt(parse_u64)(rm)?;
            rm = r;
            rm = cchar::<_, Error<_>>(',')(rm)?.0;
            let (r, max) = opt(parse_u64)(rm)?;
            rm = r;
            let rep = if min.is_some() && max.is_some() {
                Repetition::Range(min.unwrap(), max.unwrap())
            }
            else if min.is_some() {
                Repetition::AtLeast(min.unwrap())
            }
            else if max.is_some() {
                Repetition::LessThanEq(max.unwrap())
            }
            else {
                panic!() // TODO: This is accepted by https://regex101.com/ and treated as plain text (no repetition)
            };
            rm = cchar::<_, Error<_>>('}')(rm)?.0;
            rep
        }
        else if let Ok((r, _)) = cchar::<_, Error<_>>('?')(rm) {
            rm = r;
            Repetition::LessThanEq(1)
        }
        else if let Ok((r, _)) = cchar::<_, Error<_>>('+')(rm) {
            rm = r;
            Repetition::AtLeast(1)
        }
        else {
            Repetition::None
        };

        ors.push(ChainedMatchable {
            matchables: base,
            repetition,
            next: None,
        });

        if let Ok((r, _)) = cchar::<_, Error<_>>('|')(r) {
            rm = r;
            continue;
        }
        break;
    }
    let r = rm;

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

    if let Ok((r, _)) = cchar::<_, Error<_>>(')')(r) {
        return Ok((r, m))
    }

    let (r, c) = parse_regex(r)?;
    m.next = Some(Box::new(c));
    Ok((r, m))
}

