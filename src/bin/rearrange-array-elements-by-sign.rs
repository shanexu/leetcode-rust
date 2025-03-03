fn main() {
    println!("{:?}", Solution::rearrange_array(vec![3, 1, -2, -5, 2, -4]));
}

struct Solution;

impl Solution {
    pub fn rearrange_array(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ps = Vec::with_capacity(n >> 1);
        let mut ns = Vec::with_capacity(n >> 1);
        for &num in nums.iter() {
            if num > 0 {
                ps.push(num);
            } else {
                ns.push(num);
            }
        }
        for i in 0..n {
            if i & 1 == 1 {
                nums[i] = ns[i >> 1];
            } else {
                nums[i] = ps[i >> 1];
            }
        }
        nums
    }
}
