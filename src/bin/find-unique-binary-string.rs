use leetcode_rust::vec_string;

fn main() {
    println!(
        "{}",
        Solution::find_different_binary_string(vec_string!["01", "10"])
    );
    println!(
        "{}",
        Solution::find_different_binary_string(vec_string!["00", "01"])
    );
    println!(
        "{}",
        Solution::find_different_binary_string(vec_string!["111", "011", "001"])
    );
}

struct Solution;

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let n = nums.len();
        let mut ans = vec![0u8; nums.len()];
        for i in 0..n {
            let b = nums[i].as_bytes()[i];
            ans[i] = if b == b'0' { b'1' } else { b'0' }
        }
        String::from_utf8(ans).unwrap()
    }
}
