use leetcode_rust::vec_vec_i32;

fn main() {
    println!(
        "{}",
        Solution::count_days(10, vec_vec_i32![[5, 7], [1, 3], [9, 9]])
    );
}

struct Solution;

impl Solution {
    pub fn count_days(days: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        meetings.sort();
        meetings.push(vec![days + 1, days + 1]);
        let mut ans = 0;
        let mut start = 1;
        let mut i = 0;
        let n = meetings.len();
        while start <= days && i < n {
            let m = &meetings[i];
            let begin = m[0];
            let end = m[1];
            ans += begin - start;
            start = end + 1;
            i += 1;
            while i < n {
                let m = &meetings[i];
                if m[0] > start {
                    break;
                }
                start = start.max(m[1] + 1);
                i += 1;
            }
        }
        ans
    }
}
