use leetcode_rust::vec_vec_i32;

fn main() {
    assert_eq!(
        2,
        Solution::min_zero_array(vec![2, 0, 2], vec_vec_i32![[0, 2, 1], [0, 2, 1], [1, 1, 3]])
    );

    assert_eq!(
        -1,
        Solution::min_zero_array(vec![4, 3, 2, 1], vec_vec_i32![[1, 3, 2], [0, 2, 1]])
    );
}

struct Solution;

impl Solution {
    pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let m = queries.len();
        let mut diff = vec![0; n + 1];
        let mut sum = 0;
        let mut query_idx = 0;
        for i in 0..n {
            sum += diff[i];
            while nums[i] > sum {
                if query_idx == m {
                    return -1;
                }
                let q = &queries[query_idx];
                query_idx += 1;
                if i > q[1] as usize {
                    continue;
                }
                if i >= q[0] as usize {
                    sum += q[2];
                } else {
                    diff[q[0] as usize] += q[2];
                }
                diff[q[1] as usize + 1] -= q[2];
            }
        }
        query_idx as i32
    }
}
