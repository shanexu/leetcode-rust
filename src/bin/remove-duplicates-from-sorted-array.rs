fn main() {
    let mut nums = vec![];
    let n = Solution::remove_duplicates(&mut nums);
    printvec(&nums, n);
}

struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut p = nums[0];
        let mut idx: usize = 0;
        for i in 1..nums.len() {
            let x = nums[i];
            if p != x {
                p = x;
                idx += 1;
                if idx != i {
                    nums[idx] = x;
                }
            }
        }
        (idx + 1) as i32
    }
}

fn printvec(nums: &Vec<i32>, n: i32) {
    for i in 0..(n as usize) {
        println!("{}", nums[i])
    }
}
