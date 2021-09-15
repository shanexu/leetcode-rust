fn main() {
    println!("{:?}", Solution::total_n_queens(4));
}

struct Solution;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let n = n as usize;
        let mut results = 0;
        let mut candidates = vec![];
        for i in 0..n {
            candidates.push(vec![i]);
        }
        solve(&mut results, &mut candidates, n);
        results
    }
}

fn solve(results: &mut i32, candidates: &mut Vec<Vec<usize>>, n: usize) {
    if candidates.len() == 0 {
        return;
    }
    let c = candidates.pop().unwrap();
    if c.len() == n {
        *results += 1
    } else {
        let k = c.len();
        for p in 0..n {
            let mut ok = true;
            for i in 0..k {
                let j = c[i];
                if p == j {
                    ok = false;
                    break;
                }
                if j as i32 - i as i32 == p as i32 - k as i32 {
                    ok = false;
                    break;
                }
                if i + j == p + k {
                    ok = false;
                    break;
                }
            }
            if ok {
                let mut cc = c.clone();
                cc.push(p);
                candidates.push(cc);
            }
        }
    }
    solve(results, candidates, n);
}
