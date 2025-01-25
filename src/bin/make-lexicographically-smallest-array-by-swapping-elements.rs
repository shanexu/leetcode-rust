fn main() {
    println!(
        "{:?}",
        Solution::lexicographically_smallest_array(vec![1, 7, 6, 18, 2, 1], 3)
    );
    println!(
        "{:?}",
        Solution2::lexicographically_smallest_array(vec![1, 7, 6, 18, 2, 1], 3)
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
        let mut is = vec![];
        for (j, &(num, i)) in nums.iter().enumerate() {
            if num - prev > limit {
                is.sort();
                let offset = j - is.len();
                for k in 0..is.len() {
                    ans[is[k]] = nums[offset + k].0;
                }
                is.clear()
            }
            is.push(i);
            prev = num;
        }
        is.sort();
        let offset = nums.len() - is.len();
        for k in 0..is.len() {
            ans[is[k]] = nums[offset + k].0;
        }
        ans
    }
}

struct Solution2;

impl Solution2 {
    pub fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
        let n = nums.len();
        let mut sorted: Vec<usize> = (0..n).collect();
        sorted.sort_by(|i, j| nums[*i].cmp(&nums[*j]));
        let mut ans = vec![0; n];
        let mut prev = nums[sorted[0]];
        let mut is = vec![];
        for (j, &i) in sorted.iter().enumerate() {
            let num = nums[i];
            if num - prev > limit {
                is.sort();
                let offset = j - is.len();
                for k in 0..is.len() {
                    ans[is[k]] = nums[sorted[offset + k]];
                }
                is.clear()
            }
            is.push(i);
            prev = num;
        }
        is.sort();
        let offset = nums.len() - is.len();
        for k in 0..is.len() {
            ans[is[k]] = nums[sorted[offset + k]];
        }
        ans
    }
}
