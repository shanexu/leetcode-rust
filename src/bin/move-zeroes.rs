fn main() {
    let mut nums = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes(&mut nums);
    println!("{:?}", nums);
}

struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut j: usize = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums[j] = nums[i];
                j += 1;
            }
        }
        for k in j..nums.len() {
            nums[k] = 0;
        }
    }
}
