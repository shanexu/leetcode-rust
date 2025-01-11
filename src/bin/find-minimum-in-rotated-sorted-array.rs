fn main() {
    println!("{}", Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]));
}

struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            if nums[left] < nums[right] {
                break;
            }
            let mid = left + (right - left) / 2;
            let mid_val = nums[mid];
            if mid_val > nums[right] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        return nums[left];
    }
}
