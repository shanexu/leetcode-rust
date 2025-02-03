fn main() {
    assert_eq!(Solution::max_chunks_to_sorted(vec![4, 3, 2, 1, 0]), 1);
    assert_eq!(Solution::max_chunks_to_sorted(vec![1, 0, 2, 3, 4]), 4);
}

struct Solution;

impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut max = 0;
        for (i, &v) in arr.iter().enumerate() {
            max = std::cmp::max(max, v);
            if max == i as i32 {
                count += 1;
            }
        }
        count
    }
}
