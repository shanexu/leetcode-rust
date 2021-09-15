fn main() {
    println!("{:?}", Solution::combination_sum(vec![2, 3, 6, 7], 7));
}

struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut results = vec![];
        let mut candidates = candidates;
        candidates.sort();
        let candidates = candidates.as_slice();
        solve(&mut results, vec![], candidates, target);
        results
    }
}

fn solve(results: &mut Vec<Vec<i32>>, current: Vec<i32>, candidates: &[i32], target: i32) {
    if target == 0 {
        results.push(current);
        return;
    }
    for i in 0..candidates.len() {
        let c = candidates[i];
        if c > target {
            continue;
        }
        if target >= 2*c || target == c {
            let (_, cs) = candidates.split_at(i);
            let mut new_current = current.clone();
            new_current.push(c);
            solve(results, new_current, cs, target - c);
        }
    }
}
