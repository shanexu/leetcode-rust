fn main() {
    println!("{}", Solution::minimum_index(vec![1, 2, 2, 2]));
}

struct Solution;

impl Solution {
    pub fn minimum_index(nums: Vec<i32>) -> i32 {
        let x = find_x(&nums);
        let c = nums.iter().filter(|&&num| num == x).count();
        let n = nums.len();
        let mut left = 0;
        for i in 0..n - 1 {
            if x == nums[i] {
                left += 1;
            }
            if left * 2 > i + 1 && (c - left) * 2 > n - i - 1 {
                return i as i32;
            }
        }
        -1
    }
}

// Boyer-Moore Voting Algorithm
#[inline]
fn find_x(nums: &Vec<i32>) -> i32 {
    let mut candidate = 0;
    let mut votes = 0;

    for &num in nums {
        if votes == 0 {
            candidate = num;
            votes += 1;
        } else if candidate == num {
            votes += 1;
        } else {
            votes -= 1;
        }
    }
    candidate
}
