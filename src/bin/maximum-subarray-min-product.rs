fn main() {
    println!("{}", Solution::max_sum_min_product(vec![1, 2, 3, 2]));
    println!(
        "{}",
        Solution::max_sum_min_product(vec![1, 1, 3, 2, 2, 2, 1, 5, 1, 5])
    );
}

struct Solution;

impl Solution {
    pub fn max_sum_min_product(nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let n = nums.len();
        let mut prefix_sums: Vec<i64> = vec![0; nums.len()];
        prefix_sums[0] = nums[0] as i64;
        for i in 1..nums.len() {
            prefix_sums[i] = nums[i] as i64 + prefix_sums[i - 1];
        }
        let mut stack: Vec<usize> = vec![];
        let mut m: i64 = 0;
        for (i, &v) in nums.iter().enumerate() {
            while !stack.is_empty() && v < nums[stack[stack.len() - 1]] {
                let j = stack.pop().unwrap();
                m = std::cmp::max(
                    m,
                    (prefix_sums[i - 1]
                        - (if stack.is_empty() {
                            0
                        } else {
                            prefix_sums[stack[stack.len() - 1]]
                        }))
                        * nums[j] as i64,
                )
            }
            stack.push(i);
        }
        while let Some(j) = stack.pop() {
            m = std::cmp::max(
                m,
                (prefix_sums[n - 1]
                    - (if stack.is_empty() {
                        0
                    } else {
                        prefix_sums[stack[stack.len() - 1]]
                    }))
                    * nums[j] as i64,
            );
        }
        (m % MOD) as i32
    }
}
