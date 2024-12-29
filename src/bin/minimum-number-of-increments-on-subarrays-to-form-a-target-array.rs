fn main() {
    println!("{}", Solution::min_number_operations(vec![1, 2, 3, 4]));
    println!("{}", Solution::min_number_operations(vec![1, 2, 3, 2, 1]));
    println!("{}", Solution::min_number_operations(vec![3, 1, 1, 2]));
    println!("{}", Solution::min_number_operations(vec![3, 1, 5, 4, 2]));
}

struct Solution;

impl Solution {
    pub fn min_number_operations(target: Vec<i32>) -> i32 {
        let mut ans = target[0];
        for x in target.windows(2) {
            ans += 0.max(x[1] - x[0]);
        }
        ans
    }
}
