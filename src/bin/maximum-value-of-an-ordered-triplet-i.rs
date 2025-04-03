fn main() {
    assert_eq!(77, Solution::maximum_triplet_value(vec![12, 6, 1, 2, 7]));
    assert_eq!(133,Solution::maximum_triplet_value(vec![1, 10, 3, 4, 19]));
    assert_eq!(0, Solution::maximum_triplet_value(vec![1, 2, 3]));
}

struct Solution;

impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let mut imax = 0;
        let mut dmax = 0;
        let mut ans = 0;
        for num in nums {
            ans = ans.max(dmax * num as i64);
            dmax = dmax.max(imax - num as i64);
            imax = imax.max(num as i64);
        }
        ans
    }
}
