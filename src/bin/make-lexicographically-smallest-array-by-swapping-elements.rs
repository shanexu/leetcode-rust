fn main() {
    println!(
        "{:?}",
        Solution::lexicographically_smallest_array(vec![1, 7, 6, 18, 2, 1], 3)
    );
}

struct Solution;

impl Solution {
    pub fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
        let n = nums.len();
        let mut nums: Vec<(i32, usize)> =
            nums.into_iter().enumerate().map(|(i, x)| (x, i)).collect();
        nums.sort();
        let mut ans = vec![0; n];
        let mut prev = nums[0].0;
        let mut ns = vec![];
        let mut is = vec![];
        for (num, i) in nums {
            if num - prev > limit {
                is.sort();
                for k in 0..is.len() {
                    ans[is[k]] = ns[k];
                }
                ns.clear();
                is.clear()
            }
            is.push(i);
            ns.push(num);
            prev = num;
        }
        is.sort();
        for k in 0..is.len() {
            ans[is[k]] = ns[k];
        }
        ans
    }
}
