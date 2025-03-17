fn main() {
    assert_eq!(true, Solution::divide_array(vec![3, 2, 3, 2, 2, 2]));
    assert_eq!(false, Solution::divide_array(vec![1, 2, 3, 2, 2, 2]));
}

struct Solution;

impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        let mut freq: Vec<u8> = vec![0; 501];
        for num in nums {
            freq[num as usize] ^= 1;
        }
        freq.iter().all(|&v| v == 0)
    }
}
