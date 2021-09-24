fn main() {
    let valid = [
        "2",
        "0089",
        "-0.1",
        "+3.14",
        "4.",
        "-.9",
        "2e10",
        "-90E3",
        "3e+7",
        "+6e-1",
        "53.5e93",
        "-123.456e789",
    ];
    for s in valid {
        println!("{}\t{}", s, Solution::is_number(String::from(s)));
    }

    println!("---------------------");

    let invalid = ["abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53"];
    for s in invalid {
        println!("{}\t{}", s, Solution::is_number(String::from(s)));
    }
}

fn is_number(s: String) -> bool {
    let integer = vec![SIGN, '?', DIGITAL, '+', '.'];
    let decimal = vec![
        SIGN, '?', DIGITAL, '+', DOT, '.', DIGITAL, '*', '.', DOT, DIGITAL, '+', '.', '|', '.',
    ];
    let number = vec![
        decimal.clone(),
        integer.clone(),
        vec!['|'],
        vec![EXP],
        integer.clone(),
        vec!['.'],
        vec!['?'],
        vec!['.'],
    ]
    .concat();
    is_match(s, number.into_iter().collect())
}

fn is_match(s: String, p: String) -> bool {
    let mut nfa = post2nfa(p);
    nfa.matches(s)
}

fn post2nfa(str: String) -> Nfa {
    let mut stack: Vec<Frag> = vec![];
    let mut nfa = Nfa {
        states: vec![],
        start: 0,
        listid: 0,
    };
    for p in str.chars() {
        match p {
            '.' => {
                let e2 = stack.pop().unwrap();
                let mut e1 = stack.pop().unwrap();
                nfa.patch(&mut e1.out, e2.start);
                stack.push(Frag {
                    start: e1.start,
                    out: e2.out,
                });
            }
            '|' => {
                let mut e2 = stack.pop().unwrap();
                let mut e1 = stack.pop().unwrap();
                (&mut e1.out).append(&mut e2.out);
                let s = nfa.state(SPLIT, Some(e1.start), Some(e2.start));
                stack.push(Frag {
                    start: s,
                    out: e1.out,
                });
            }
            '?' => {
                let mut e = stack.pop().unwrap();
                let s = nfa.state(SPLIT, Some(e.start), None);
                e.out.push((s, StateOut::Out1));
                stack.push(Frag {
                    start: s,
                    out: e.out,
                });
            }
            '*' => {
                let mut e = stack.pop().unwrap();
                let id = nfa.state(SPLIT, Some(e.start), None);
                nfa.patch(&mut e.out, id);
                stack.push(Frag {
                    start: id,
                    out: vec![(id, StateOut::Out1)],
                });
            }
            '+' => {
                let mut e = stack.pop().unwrap();
                let id = nfa.state(SPLIT, Some(e.start), None);
                nfa.patch(&mut e.out, id);
                stack.push(Frag {
                    start: e.start,
                    out: vec![(id, StateOut::Out1)],
                });
            }
            _ => {
                let id = nfa.state(p, None, None);
                stack.push(Frag {
                    start: id,
                    out: vec![(id, StateOut::Out)],
                })
            }
        }
    }

    let mut e = stack.pop().unwrap();
    let id = nfa.state(MATCH, None, None);
    nfa.patch(&mut e.out, id);
    nfa.start = e.start;
    println!("{:?}", nfa);
    nfa
}

struct Solution;

impl Solution {
    pub fn is_number(s: String) -> bool {
        let mut nfa = Nfa {
            states: vec![
                State {
                    c: 'ă',
                    out: Some(10),
                    out1: None,
                    lastlist: 0,
                },
                State {
                    c: 'ā',
                    out: Some(0),
                    out1: Some(10),
                    lastlist: 0,
                },
                State {
                    c: 'Ă',
                    out: Some(3),
                    out1: None,
                    lastlist: 0,
                },
                State {
                    c: 'ā',
                    out: Some(2),
                    out1: Some(4),
                    lastlist: 0,
                },
                State {
                    c: 'ą',
                    out: Some(6),
                    out1: None,
                    lastlist: 0,
                },
                State {
                    c: 'Ă',
                    out: Some(6),
                    out1: None,
                    lastlist: 0,
                },
                State {
                    c: 'ā',
                    out: Some(5),
                    out1: Some(21),
                    lastlist: 0,
                },
                State {
                    c: 'ą',
                    out: Some(8),
                    out1: None,
                    lastlist: 0,
                },
                State {
                    c: 'Ă',
                    out: Some(9),
                    out1: None,
                    lastlist: 0,
                },
                State {
                    c: 'ā',
                    out: Some(8),
                    out1: Some(21),
                    lastlist: 0,
                },
                State {
                    c: 'ā',
                    out: Some(2),
                    out1: Some(7),
                    lastlist: 0,
                },
                State {
                    c: 'ă',
                    out: Some(13),
                    out1: None,
                    lastlist: 0,
                },
                State {
                    c: 'ā',
                    out: Some(11),
                    out1: Some(13),
                    lastlist: 0,
                },
                State {
                    c: 'Ă',
                    out: Some(14),
                    out1: None,
                    lastlist: 0,
                },
                State {
                    c: 'ā',
                    out: Some(13),
                    out1: Some(21),
                    lastlist: 0,
                },
                State {
                    c: 'ā',
                    out: Some(1),
                    out1: Some(12),
                    lastlist: 0,
                },
                State {
                    c: 'Ą',
                    out: Some(18),
                    out1: None,
                    lastlist: 0,
                },
                State {
                    c: 'ă',
                    out: Some(19),
                    out1: None,
                    lastlist: 0,
                },
                State {
                    c: 'ā',
                    out: Some(17),
                    out1: Some(19),
                    lastlist: 0,
                },
                State {
                    c: 'Ă',
                    out: Some(20),
                    out1: None,
                    lastlist: 0,
                },
                State {
                    c: 'ā',
                    out: Some(19),
                    out1: Some(22),
                    lastlist: 0,
                },
                State {
                    c: 'ā',
                    out: Some(16),
                    out1: Some(22),
                    lastlist: 0,
                },
                State {
                    c: 'Ā',
                    out: None,
                    out1: None,
                    lastlist: 0,
                },
            ],
            start: 15,
            listid: 0,
        };
        nfa.matches(s)
    }
}

const MATCH: char = '\u{0100}';
const SPLIT: char = '\u{0101}';
const DIGITAL: char = '\u{0102}';
const SIGN: char = '\u{0103}';
const EXP: char = '\u{0104}';
const DOT: char = '\u{0105}';

#[derive(Debug)]
enum Matcher {
    Match,
    Split,
    Digital,
    Sign,
    Exp,
    Dot,
    Char(char),
}

impl Solution {}

#[derive(Debug)]
struct Nfa {
    states: Vec<State>,
    start: usize,
    listid: usize,
}

use std::mem::swap;

impl Nfa {
    fn patch(&mut self, l: &mut Ptrlist, s: usize) {
        for (id, out) in l.iter() {
            let st: &mut State = self.states.get_mut(*id).unwrap();
            match out {
                StateOut::Out => st.out = Some(s),
                StateOut::Out1 => st.out1 = Some(s),
            }
        }
    }

    fn state(&mut self, c: char, out: Option<usize>, out1: Option<usize>) -> usize {
        let id = self.states.len();
        let s = State {
            c,
            out,
            out1,
            lastlist: 0,
        };
        self.states.push(s);
        id
    }

    fn matches(&mut self, str: String) -> bool {
        let mut clist = vec![];
        let mut nlist = vec![];
        self.start_list(&mut clist);
        for c in str.chars() {
            self.step(&mut clist, c, &mut nlist);
            swap(&mut clist, &mut nlist);
        }
        self.is_match(&clist)
    }

    fn start_list(&mut self, l: &mut Vec<usize>) {
        l.clear();
        self.listid += 1;
        self.addstate(l, self.start);
    }

    fn is_match(&self, clist: &Vec<usize>) -> bool {
        for &id in clist.iter() {
            if self.states[id].c == MATCH {
                return true;
            }
        }
        false
    }

    fn step(&mut self, clist: &mut Vec<usize>, c: char, nlist: &mut Vec<usize>) {
        nlist.clear();
        self.listid += 1;
        for &idx in clist.iter() {
            let s = self.states[idx].clone();
            if match s.c {
                DIGITAL => c.is_ascii_digit(),
                SIGN => c == '+' || c == '-',
                EXP => c == 'e' || c == 'E',
                DOT => c == '.',
                _ => s.c == c,
            } {
                if s.out.is_some() {
                    self.addstate(nlist, s.out.unwrap());
                }
            }
        }
    }

    fn addstate(&mut self, l: &mut Vec<usize>, id: usize) {
        if self.states[id].lastlist == self.listid {
            return;
        }
        self.states[id].lastlist = self.listid;
        let s = &self.states[id].clone();
        if s.c == SPLIT {
            if s.out.is_some() {
                self.addstate(l, s.out.unwrap());
            }
            if s.out1.is_some() {
                self.addstate(l, s.out1.unwrap());
            }
            return;
        }
        l.push(id);
    }
}

#[derive(Debug, Clone)]
struct State {
    c: char,
    out: Option<usize>,
    out1: Option<usize>,
    lastlist: usize,
}

#[derive(Debug)]
enum StateOut {
    Out,
    Out1,
}

type Ptrlist = Vec<(usize, StateOut)>;

struct Frag {
    start: usize,
    out: Ptrlist,
}
