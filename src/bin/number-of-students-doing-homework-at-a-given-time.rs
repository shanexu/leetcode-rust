fn main() {
    println!(
        "{}",
        Solution::busy_student(vec![1, 2, 3], vec![3, 2, 7], 4)
    );
    println!("{}", Solution::busy_student(vec![4], vec![4], 4));
}

struct Solution;

impl Solution {
    pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        let mut ans = 0;
        for i in 0..start_time.len() {
            if start_time[i] <= query_time && query_time <= end_time[i] {
                ans += 1;
            }
        }
        ans
    }
}
