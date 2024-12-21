fn main() {
    // println!("{}", Solution::find_unsorted_subarray(vec![2,6,4,8,10,9,15]));
    println!("{}", Solution::find_unsorted_subarray(vec![2, 3, 3]));
}

struct Solution;

impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let mut l = i32::MAX;
        let mut r = i32::MAX;
        let mut m = i32::MIN;
        let mut stack: Vec<usize> = vec![];
        for i in 0..nums.len() {
            // 删除栈顶上比当前值大的索引
            while !stack.is_empty() && nums[stack[stack.len() - 1]] > nums[i] {
                stack.pop();
            }
            if !stack.is_empty() {
                // 当前索引和左边比当前值小的索引的差 > 1 说明中间有比当前值大的数字，
                // 当前值需要排在寨顶之后，那说明寨顶+1到i是未排序的
                if i - stack[stack.len() - 1] > 1 {
                    l = std::cmp::min(l, stack[stack.len() - 1] as i32);
                    r = i as i32;
                } else {
                    // 当前值 < 最大值，说明排序的时候至少要把最大值放到当前索引
                    if nums[i] < m {
                        r = i as i32;
                    }
                    // 当前值等于最大值，但是栈顶值不是最大值，那么需要调整将之前的最大值，
                    // 放置到当前值左边，所以是 i-1，例如用 [3,1,2,3]
                    // 如果栈顶值和最大值相同，说明排序是正确的，不需要做什么操作
                    else if nums[stack[stack.len() - 1]] != m {
                        r = (i - 1) as i32;
                    }
                    // 当前值大于最大值，说明当前值的位置是对的，所以不对r进行操作
                    // else {}
                }
            } else {
                if i != 0 {
                    l = -1;
                    r = i as i32;
                }
            }
            stack.push(i);
            m = std::cmp::max(m, nums[i]);
        }
        r - l
    }
}
