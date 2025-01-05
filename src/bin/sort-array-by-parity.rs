fn main() {
    println!("{:?}", Solution::sort_array_by_parity(vec![3, 1, 2, 4]));
}

struct Solution;

impl Solution {
    pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
        let mut i = 0;
        let mut j = nums.len() - 1;
        while i < j {
            if nums[i] % 2 == 0 {
                i += 1;
            } else if nums[j] % 2 == 1 {
                j -= 1;
            } else {
                nums.swap(i, j);
                i += 1;
                j -= 1;
            }
        }
        nums
    }
}
