fn main() {
    println!(
        "{}",
        Solution::is_match(String::from("aa"), String::from("a"))
    );
    println!(
        "{}",
        Solution::is_match(String::from("aa"), String::from("*"))
    );
    println!(
        "{}",
        Solution::is_match(String::from("cb"), String::from("?a"))
    );
    println!(
        "{}",
        Solution::is_match(String::from("adceb"), String::from("*a*b"))
    );
    println!(
        "{}",
        Solution::is_match(String::from("acdcb"), String::from("a*c?b"))
    );
}

struct Solution {}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        if p.len() == 0 {
            return s.len() == 0;
        }
        let mut nfa = pattern2nfa(p);
        nfa.matches(s)
    }
}

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
            if s.c == c || s.c == '?' {
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

const MATCH: char = '\u{0100}';
const SPLIT: char = '\u{0101}';

fn pattern2nfa(str: String) -> Nfa {
    let mut stack: Vec<Frag> = vec![];
    let mut nfa = Nfa {
        states: vec![],
        start: 0,
        listid: 0,
    };
    for p in str.chars() {
        match p {
            '*' => {
                {
                    let id = nfa.state('?', None, None);
                    stack.push(Frag {
                        start: id,
                        out: vec![(id, StateOut::Out)],
                    });
                }

                let mut e = stack.pop().unwrap();
                let id = nfa.state(SPLIT, Some(e.start), None);
                nfa.patch(&mut e.out, id);
                stack.push(Frag {
                    start: id,
                    out: vec![(id, StateOut::Out1)],
                });
            }
            _ => {
                let id = nfa.state(p, None, None);
                stack.push(Frag {
                    start: id,
                    out: vec![(id, StateOut::Out)],
                });
            }
        }
    }

    while stack.len() > 1 {
        let e2 = stack.pop().unwrap();
        let mut e1 = stack.pop().unwrap();
        nfa.patch(&mut e1.out, e2.start);
        stack.push(Frag {
            start: e1.start,
            out: e2.out,
        });
    }

    let mut e = stack.pop().unwrap();
    let id = nfa.state(MATCH, None, None);
    nfa.patch(&mut e.out, id);
    nfa.start = e.start;
    nfa
}
