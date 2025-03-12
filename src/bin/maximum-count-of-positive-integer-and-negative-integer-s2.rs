fn main() {
    assert_eq!(3, Solution::maximum_count(vec![-2, -1, -1, 1, 2, 3]));
    assert_eq!(3, Solution::maximum_count(vec![-3, -2, -1, 0, 0, 1, 2]));
    assert_eq!(4, Solution::maximum_count(vec![0, 5, 20, 66, 1314]));
    assert_eq!(4, Solution::maximum_count(vec![5, 20, 66, 1314]));
    assert_eq!(6, Solution::maximum_count(vec![-1, -2, -3, -4, -5, -6]));
    assert_eq!(0, Solution::maximum_count(vec![0, 0]));
}

struct Solution;
impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let n = nums.len();
        let mut size = n;
        while size > 1 {
            let half = size / 2;
            let mid = left + half;
            if nums[mid] < 0 {
                left = mid;
            }
            size -= half;
        }
        let neg = if nums[left] < 0 { left + 1 } else { 0 };
        size = n - left;
        while size > 1 {
            let half = size / 2;
            let mid = left + half;
            if nums[mid] <= 0 {
                left = mid;
            }
            size -= half;
        }
        let pos = if nums[left] <= 0 {
            n - left - 1
        } else {
            n - left
        };
        neg.max(pos) as i32
    }
}
