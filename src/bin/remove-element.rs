fn main() {
    let mut nums = vec![1, 3, 3];
    let n = Solution::remove_element(&mut nums, 3);
    println!("n={}", n);
    println!("{:?}", nums);
}

struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut idx: usize = 0;
        for i in 0..nums.len() {
            let x = nums[i];
            if x != val {
                if idx != i {
                    nums[idx] = x;
                }
                idx += 1;
            }
        }
        idx as i32
    }
}
