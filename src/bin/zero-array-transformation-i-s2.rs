use leetcode_rust::vec_vec_i32;

fn main() {
    assert!(Solution::is_zero_array(vec![1, 0, 1], vec_vec_i32![[0, 2]]));
}

struct Solution;

impl Solution {
    pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
        let n = nums.len();
        let m = queries.len();
        let mut diff = vec![0; n + 1];
        let mut query_idx = 0;
        let mut sum = 0;
        for i in 0..n {
            sum += diff[i];
            while nums[i] > sum {
                if query_idx == m {
                    return false;
                }
                let q = &queries[query_idx];
                query_idx += 1;
                if i > q[1] as usize {
                    continue;
                }
                if i >= q[0] as usize {
                    sum += 1;
                } else {
                    diff[q[0] as usize] += 1;
                }
                diff[q[1] as usize + 1] -= 1;
            }
        }
        true
    }
}
