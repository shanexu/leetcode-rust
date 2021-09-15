fn main() {
    println!("{:?}", Solution::permute(vec![1, 2, 3, 4]))
}

struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut results = Vec::with_capacity(size(nums.len()));
        let mut nums = nums;
        solve(&mut results, &mut nums, 0);
        results
    }
}

fn size(n: usize) -> usize {
    let mut p = 1;
    for i in 1..n + 1 {
        p = p * i;
    }
    p
}

fn solve(results: &mut Vec<Vec<i32>>, nums: &mut Vec<i32>, step: usize) {
    if step == nums.len() - 1 {
        results.push(nums.clone());
        return;
    }
    for i in step..nums.len() {
        nums.swap(step, i);
        solve(results, nums, step + 1);
        nums.swap(step, i);
    }
}
