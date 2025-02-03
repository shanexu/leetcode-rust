fn main() {
    println!("{}", Solution::duplicate_numbers_xor(vec![1,2,1,3]));
}

struct Solution;

impl Solution {
    pub fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut freq = vec![0; 51];
        for num in nums {
            freq[num as usize] += 1;
            if freq[num as usize] == 2 {
                ans ^= num;
            }
        }
        ans
    }
}
