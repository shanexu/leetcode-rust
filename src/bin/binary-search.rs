fn main() {
    assert_eq!(4, Solution::search(vec![-1, 0, 3, 5, 9, 12], 9));
    assert_eq!(-1, Solution::search(vec![-1, 0, 3, 5, 9, 12], 2));
}

struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut size = nums.len();
        let mut base = 0;
        while size > 1 {
            let half = size / 2;
            let mid = base + half;
            let cmp = nums[mid] - target;
            base = if cmp > 0 { base } else { mid };
            size -= half;
        }
        if nums[base] == target {
            base as i32
        } else {
            -1
        }
    }
}
