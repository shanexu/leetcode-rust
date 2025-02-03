use leetcode_rust::vec_char;

fn main() {
    println!(
        "{}",
        Solution::least_interval(vec_char!["A", "A", "A", "B", "B", "B"], 2)
    )
}

struct Solution;

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut freq = [0; 26];
        let mut max = 0;
        for &c in tasks.iter() {
            let idx = c as usize - 'A' as usize;
            freq[idx] += 1;
            max = max.max(freq[idx]);
        }
        let mut max_count = 0;
        for c in freq {
            if c == max {
                max_count += 1;
            }
        }
        std::cmp::max(tasks.len() as i32, (max - 1) * (n + 1) + max_count)
    }
}
