fn main() {
    println!(
        "{}",
        Solution::single_non_duplicate(vec![1, 1, 2, 3, 3, 4, 4, 8, 8])
    );
    println!("{}", Solution::single_non_duplicate(vec![1]));
    println!("{}", Solution::single_non_duplicate(vec![0, 0, 1]));
    println!("{}", Solution::single_non_duplicate(vec![1, 2, 2]));
    println!(
        "{}",
        Solution::single_non_duplicate(vec![3, 3, 7, 7, 10, 11, 11])
    );
}

struct Solution;

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut size = nums.len();
        while size > 1 {
            let k = size >> 1;
            let k_is_odd = k & 1 == 1;
            let mid = left + k;
            if nums[mid - 1] == nums[mid] {
                if k_is_odd {
                    left = mid + 1;
                    size = k;
                } else {
                    size = k - 1;
                }
            } else if nums[mid] == nums[mid + 1] {
                if k_is_odd {
                    size = k;
                } else {
                    left = mid + 2;
                    size = k - 1;
                }
            } else {
                return nums[mid];
            }
        }
        nums[left]
    }
}
