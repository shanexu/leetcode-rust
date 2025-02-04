fn main() {
    assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    assert_eq!(Solution2::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    assert_eq!(Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
    assert_eq!(Solution2::length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
    assert_eq!(Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1);
    assert_eq!(Solution2::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1);
    assert_eq!(Solution::length_of_lis(vec![4, 10, 4, 3, 8, 9]), 3);
    assert_eq!(Solution2::length_of_lis(vec![4, 10, 4, 3, 8, 9]), 3);
    assert_eq!(Solution::length_of_lis(vec![2, 2]), 1);
    assert_eq!(Solution2::length_of_lis(vec![2, 2]), 1);
    assert_eq!(Solution::length_of_lis(vec![1, 3, 6, 7, 9, 4, 10, 5, 6]), 6);
    assert_eq!(Solution2::length_of_lis(vec![1, 3, 6, 7, 9, 4, 10, 5, 6]), 6);
    assert_eq!(Solution::length_of_lis(vec![1, 3, 6, 7, 9, 4, 10, 5, 6]), 6);
}

/// https://www.geeksforgeeks.org/longest-monotonically-increasing-subsequence-size-n-log-n/
struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut ans = Vec::with_capacity(nums.len());
        ans.push(nums[0]);
        for &num in nums.iter().skip(1) {
            if ans[ans.len() - 1] < num {
                ans.push(num);
            } else {
                let j = ans.binary_search(&num).unwrap_or_else(|x| x);
                ans[j] = num;
            }
        }
        ans.len() as i32
    }
}

/// 这个思路跟 ```maximal-rectangle``` 有点类似：
/// 最值就是，以 nums\[i\] 为最大值的中的一个
/// 定义以i为最大值的最大递增子序列函数为f
/// 那么数组nums的最大递增子序列就是 f(i) 中的最大值。
struct Solution2;

impl Solution2 {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp = vec![1; nums.len()];
        let n = nums.len();
        for i in 1..n {
            for j in 0..i {
                if nums[i] > nums[j] {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
        }
        *dp.iter().max().unwrap()
    }
}
