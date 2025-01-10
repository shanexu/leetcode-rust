fn main() {
    println!("{:?}", Solution::summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]));
}

struct Solution;

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let n = nums.len();
        if n == 0 {
            return vec![];
        }
        let mut ans = vec![];
        let mut prev = nums[0];
        let mut start = prev;
        for &num in nums.iter().skip(1) {
            if num - prev == 1 {
                prev = num;
            } else {
                if start == prev {
                    ans.push(format!("{}", start));
                } else {
                    ans.push(format!("{}->{}", start, prev));
                }
                prev = num;
                start = num;
            }
        }
        if start == prev {
            ans.push(format!("{}", start));
        } else {
            ans.push(format!("{}->{}", start, prev));
        }
        ans
    }
}
