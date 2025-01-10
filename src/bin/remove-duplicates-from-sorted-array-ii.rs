fn main() {
    println!(
        "{}",
        Solution::remove_duplicates(&mut vec![0, 0, 1, 1, 1, 1, 2, 3, 3])
    );
}

struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let n = nums.len();
        let mut j: usize = 0;
        let mut prev = 10001;
        let mut prev_count = 1;
        for i in 0..n {
            let num = nums[i];
            if num == prev {
                if prev_count == 2 {
                    continue;
                }
                prev_count += 1;
            } else {
                prev = num;
                prev_count = 1;
            }
            nums[j] = num;
            j += 1;
        }
        j as i32
    }
}
