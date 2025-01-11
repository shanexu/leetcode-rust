fn main() {
    println!("{:?}", Solution::subsets(vec![1, 2, 3]));
    println!("{:?}", Solution2::subsets(vec![1, 2, 3]));
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

struct Solution2;
impl Solution2 {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        let m = 1 << n;
        let mut ans = Vec::with_capacity(m);
        for mut i in 0..m {
            let mut s = vec![];
            let mut j = 0;
            while i > 0 {
                if i & 1 == 1 {
                    s.push(nums[j]);
                }
                j += 1;
                i >>= 1;
            }
            ans.push(s);
        }
        ans
    }
}
