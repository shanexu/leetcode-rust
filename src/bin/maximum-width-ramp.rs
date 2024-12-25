fn main() {
    println!("{}", Solution::max_width_ramp(vec![6, 0, 8, 2, 1, 5]));
}

/// 这个题目的本质就是，假设当前位置为i,找到距离当前位置最远的大于等于当前值的索引j。
/// 考虑一个情况，索引为i, nums\[i\] <= nums\[i+1\] ，从数组以从右往左的方向遍历找到第一个
/// 符合条件的j，也就是 nums\[i\] <= nums\[j\]。这里有两种情况需要讨论：\
/// 1. nums\[i+1\] <= nums\[j\]，因为j - i > j - i - 1，没必要计算i+1；
/// 2. 如果存在k，使得nums\[k\] >= nums\[i+1\]，因为nums\[i+1\] >= nums\[i\] 所以 nums\[k\] >= nums\[i\],
///    k必然 <= j，否则矛盾。
/// 综上两条，当有nums\[i\] <= nums\[i+1\]，只需计算i，不需要计算i+1。所以算法的第一步，是维护一个索引栈，如果为空或者栈顶值大于当前值则，入栈。
/// 然后从右往左遍历逐步找到i对应的j值。
struct Solution;

impl Solution {
    pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
        let mut stack: Vec<usize> = vec![];
        for (i, &v) in nums.iter().enumerate() {
            if stack.is_empty() || nums[stack[stack.len() - 1]] > v {
                stack.push(i);
            }
        }
        let mut m = 0;
        for (j, &v) in nums.iter().enumerate().rev() {
            while !stack.is_empty() && nums[stack[stack.len() - 1]] <= v {
                let i = stack.pop().unwrap();
                m = std::cmp::max(m, j - i);
            }
            if stack.is_empty() {
                break;
            }
        }
        m as i32
    }
}
