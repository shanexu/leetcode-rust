fn main() {}

struct Solution;

impl Solution {
    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n1 = nums1.len();
        let n2 = nums2.len();
        let mut ans = 0;
        if n2 & 1 == 1 {
            for num in nums1 {
                ans ^= num;
            }
        }
        if n1 & 1 == 1 {
            for num in nums2 {
                ans ^= num;
            }
        }
        ans
    }
}