fn main() {
    println!(
        "{}",
        Solution::number_of_weak_characters(vec![vec![1, 1], vec![2, 1], vec![2, 2], vec![1, 2]])
    );
    println!(
        "{}",
        Solution::number_of_weak_characters(vec![vec![10, 2], vec![10, 1]])
    );
}

struct Solution;

impl Solution {
    pub fn number_of_weak_characters(mut properties: Vec<Vec<i32>>) -> i32 {
        properties.sort_by(|a, b| a[0].cmp(&b[0]).reverse().then(a[1].cmp(&b[1])));
        let mut count = 0;
        let mut max_defense = 0;
        for c in properties.iter() {
            if c[1] < max_defense {
                count += 1;
            } else {
                max_defense = c[1];
            }
        }
        count
    }
}
