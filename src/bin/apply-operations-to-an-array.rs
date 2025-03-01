fn main() {
    assert_eq!(
        vec![1, 4, 2, 0, 0, 0],
        Solution::apply_operations(vec![1, 2, 2, 1, 1, 0])
    );
}

struct Solution;

impl Solution {
    pub fn apply_operations(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let n = nums.len();
        for i in 0..n - 1 {
            if nums[i] == nums[i + 1] {
                nums[i] *= 2;
                nums[i + 1] = 0;
            }
        }
        let mut j = 0;
        for i in 0..n {
            if nums[i] != 0 {
                nums.swap(i, j);
                j += 1;
            }
        }
        for i in j..n {
            nums[i] = 0;
        }
        nums
    }
}
