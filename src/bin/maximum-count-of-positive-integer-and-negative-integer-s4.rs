fn main() {
    assert_eq!(3, Solution::maximum_count(vec![-2, -1, -1, 0, 1, 2, 3]));
    assert_eq!(3, Solution::maximum_count(vec![-3, -2, -1, 0, 0, 1, 2]));
    assert_eq!(4, Solution::maximum_count(vec![0, 0, 5, 20, 66, 1314]));
    assert_eq!(4, Solution::maximum_count(vec![5, 20, 66, 1314]));
    assert_eq!(6, Solution::maximum_count(vec![-1, -2, -3, -4, -5, -6]));
    assert_eq!(
        6,
        Solution::maximum_count(vec![-1, -2, -3, -4, -5, -6, 0, 0])
    );
}

struct Solution;
impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let pos_zero = nums
            .binary_search_by(|a| {
                if *a <= 0 {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            })
            .unwrap_err();
        let neg_zero = nums
            .binary_search_by(|a| {
                if *a >= 0 {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Less
                }
            })
            .unwrap_err();
        (n - pos_zero).max(neg_zero) as i32
    }
}
