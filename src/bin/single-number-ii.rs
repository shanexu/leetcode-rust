fn main() {}

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