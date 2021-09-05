fn main() {
    println!("{:?}", Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]));
}

struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut results = Vec::new();
        if nums.len() < 3 {
            return results;
        }
        let mut nums = nums;
        nums.sort();
        let n = nums.len();
        let mut x = i32::MIN;
        for i in 0..(n - 2) {
            let mut l = i + 1;
            let mut r = n - 1;
            let v = nums[i];
            if v == x {
                continue;
            }
            x = v;
            let mut y0 = i32::MIN;
            while l < r {
                let y = nums[l];
                let z = nums[r];
                let s = x + y + z;
                if s == 0 {
                    if y0 != y {
                        results.push(vec![x, y, z]);
                        y0 = y;
                    }
                    l += 1;
                    r -= 1;
                } else if s < 0 {
                    l += 1;
                } else {
                    r -= 1;
                }
            }
        }
        results
    }
}
