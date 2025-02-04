fn main() {
    assert_eq!(Solution::single_number(vec![2, 2, 3, 2]), 3);
    assert_eq!(Solution::single_number(vec![0, 1, 0, 1, 0, 1, 99]), 99);
    assert_eq!(Solution::single_number(vec![30000, 500, 100, 30000, 100, 30000, 100]), 500);
    assert_eq!(Solution::single_number(vec![1, 1, 1, 2]), 2);
    assert_eq!(Solution::single_number(vec![1, 1, 1, 2, 2, 2, 3]), 3);
    assert_eq!(Solution::single_number(vec![1, 1, 1, 2, 2, 2, 3, 3, 3, 4]), 4);
}

/// 参考 https://algo.monster/liteproblems/137
struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut bit1 = 0;
        let mut bit2 = 0;

        for &num in nums.iter() {
            let new_bit1 = (!bit1 & bit2 & num) | (bit1 & !bit2 & !num);
            let new_bit2 = !bit1 & (bit2 ^ num);

            bit1 = new_bit1;
            bit2 = new_bit2;
        }

        bit2
    }
}
