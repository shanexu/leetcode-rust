fn main() {
    println!("{:?}", Solution::solve_n_queens(4));
}

struct Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;
        let mut results = vec![];
        let mut candidates = vec![];
        for i in 0..n {
            candidates.push(vec![i]);
        }
        solve(&mut results, &mut candidates, n);
        let mut str_results = Vec::with_capacity(results.len());
        for r in results.iter() {
            let mut strs = Vec::with_capacity(n);
            for &i in r.iter() {
                let mut bs = vec![b'.'; n];
                bs[i] = b'Q';
                strs.push(String::from_utf8(bs).unwrap());
            }
            str_results.push(strs);
        }
        str_results
    }
}

// 其实这个算法就是一个穷举的暴力算法
fn solve(results: &mut Vec<Vec<usize>>, candidates: &mut Vec<Vec<usize>>, n: usize) {
    if candidates.len() == 0 {
        return;
    }
    let c = candidates.pop().unwrap();
    if c.len() == n {
        results.push(c);
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
