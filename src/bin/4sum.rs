fn main() {
    println!("{:?}", Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0));
    println!("{:?}", Solution::four_sum(vec![2, 2, 2, 2, 2, 2], 8));
    println!(
        "{:?}",
        Solution::four_sum(vec![-5, -4, -3, -2, -1, 0, 0, 1, 2, 3, 4, 5], 0)
    );
}

struct Solution {}

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut results = Vec::new();
        let n = nums.len();
        if n < 4 {
            return results;
        }
        let mut nums = nums;
        nums.sort();
        let mut prev_xi = i32::MIN;
        for i in 0..(n - 3) {
            let xi = nums[i];
            if xi == prev_xi {
                continue;
            } else {
                prev_xi = xi;
            }
            let mut prev_xj = i32::MIN;
            for j in (i + 1)..(n - 2) {
                let xj = nums[j];
                if xj == prev_xj {
                    continue;
                } else {
                    prev_xj = xj;
                }
                let mut l = j + 1;
                let mut r = n - 1;
                let mut prev_xl = i32::MIN;
                while l < r {
                    let xl = nums[l];
                    let xr = nums[r];
                    let sum = xi + xj + xl + xr;
                    if sum < target {
                        l += 1;
                    } else if sum == target {
                        if prev_xl != xl {
                            results.push(vec![xi, xj, xl, xr]);
                            prev_xl = xl;
                        }
                        l += 1;
                        r -= 1;
                    } else {
                        r -= 1;
                    }
                }
            }
        }
        results
    }
}
