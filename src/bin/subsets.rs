fn main() {
    println!("{:?}", Solution::subsets(vec![1, 2, 3]));
}

struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn helper(nums: &[i32], i: usize, curr: &mut Vec<i32>) -> Vec<Vec<i32>> {
            if i == nums.len() {
                return vec![curr.clone()];
            }
            curr.push(nums[i]);
            let mut x = helper(nums, i + 1, curr);
            curr.pop();
            let y = helper(nums, i + 1, curr);
            x.extend(y);
            x
        }
        helper(&nums, 0, &mut vec![])
    }
}
