fn main() {
    println!("{:?}", Solution::min_operations("101011".to_string()));
}

struct Solution;

impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let boxes = boxes.as_bytes();
        let mut distance = 0;
        let mut right_ones = 0;
        let mut left_ones = 0;
        let mut ans = vec![0; boxes.len()];
        for (i, &b) in boxes.iter().enumerate() {
            if b == b'1' {
                right_ones += 1;
                distance += i as i32;
                distance += 1;
            }
        }
        for (i, &b) in boxes.iter().enumerate() {
            distance += left_ones - right_ones;
            ans[i] = distance;
            if b == b'1' {
                left_ones += 1;
                right_ones -= 1;
            }
        }
        ans
    }
}
