fn main() {}

struct Solution;

impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let mut freq = vec![0; 101];
        for num in nums {
            freq[num as usize] += 1;
        }
        freq.iter()
            .enumerate()
            .filter_map(|(i, &v)| if v == 1 { Some(i as i32) } else { None })
            .sum()
    }
}
