fn main() {
    println!(
        "{}",
        Solution::find_length_of_shortest_subarray(vec![1, 2, 3, 10, 4, 2, 3, 5])
    );
    println!(
        "{}",
        Solution::find_length_of_shortest_subarray(vec![3, 2, 1])
    );
}

struct Solution;

impl Solution {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut l = 0;
        let mut prev = i32::MIN;
        for (i, &v) in arr.iter().enumerate() {
            if v >= prev {
                l = i;
                prev = v;
            } else {
                break;
            }
        }
        if l == n - 1 {
            return 0;
        }
        let mut r = n - 1;
        prev = i32::MAX;
        for (i, &v) in arr.iter().enumerate().rev() {
            if v <= prev {
                r = i;
                prev = v;
            } else {
                break;
            }
        }
        let mut min_len = (n - l - 1).min(r);
        let mut j = r;
        for i in 0..=l {
            if arr[i] <= arr[j] {
                min_len = min_len.min(j - i - 1);
            } else if j < n - 1 {
                j += 1;
            } else {
                break;
            }
        }
        min_len as i32
    }
}
