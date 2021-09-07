fn main() {
    println!("{:?}", Solution::first_missing_positive(vec![3, 4, -1, 1]));
}

struct Solution {}

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut j = 0;
        for i in 0..nums.len() {
            if nums[i] <= 0 {
                let t = nums[i];
                nums[i] = nums[j];
                nums[j] = t;
                j += 1;
            }
        }

        let size = nums.len() - j;
        for i in j..nums.len() {
            let v = abs(nums[i]);
            if v - 1 < size && nums[v - 1 + j] > 0 {
                nums[v - 1 + j] = -nums[v - 1 + j]
            }
        }

        for i in j..nums.len() {
            if nums[i] > 0 {
                return (i - j + 1) as i32;
            }
        }
        (size + 1) as i32
    }
}

fn abs(x: i32) -> usize {
    if x < 0 {
        (-x) as usize
    } else {
        x as usize
    }
}
