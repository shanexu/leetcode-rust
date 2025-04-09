fn main() {
    assert_eq!(6, Solution::subset_xor_sum(vec![1, 3]));
    assert_eq!(28, Solution::subset_xor_sum(vec![5, 1, 6]));
}

struct Solution;

impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let mut subsets = vec![];
        generate_subsets(&nums, 0, &mut vec![], &mut subsets);
        let mut ans = 0;
        for subset in subsets.iter() {
            let mut x = 0;
            for &num in subset {
                x ^= num;
            }
            ans += x;
        }
        ans
    }
}

fn generate_subsets(
    nums: &Vec<i32>,
    idx: usize,
    subset: &mut Vec<i32>,
    subsets: &mut Vec<Vec<i32>>,
) {
    if idx == nums.len() {
        subsets.push(subset.clone());
        return;
    }
    subset.push(nums[idx]);
    generate_subsets(nums, idx + 1, subset, subsets);
    subset.pop();
    generate_subsets(nums, idx + 1, subset, subsets);
}
