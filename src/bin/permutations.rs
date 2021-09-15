fn main() {
    println!("{:?}", Solution::permute(vec![1,2,3,4]))
}

struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let step = nums.len() - 1;
        let mut results = vec![];
        let mut nums = nums;
        solve(&mut results, &mut nums, step);
        results
    }
}

fn solve(results: &mut Vec<Vec<i32>>, nums: &mut Vec<i32>, step: usize) {
    if step == 0 {
        results.push(nums.clone());
        return
    }
    for i in (0..(step+1)).rev() {
        nums.swap(step, i);
        solve(results, nums, step - 1);
        nums.swap(step, i);
    }
}
