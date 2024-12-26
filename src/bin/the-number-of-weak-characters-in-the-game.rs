use std::collections::VecDeque;

fn main() {
    println!(
        "{}",
        Solution::number_of_weak_characters(vec![vec![1, 1], vec![2, 1], vec![2, 2], vec![1, 2]])
    );
}

struct Solution;

impl Solution {
    pub fn number_of_weak_characters(mut properties: Vec<Vec<i32>>) -> i32 {
        properties.sort_by(|a, b| a[0].cmp(&b[0]).then(a[1].cmp(&b[1])));
        let mut count = 0;
        let mut stacks: std::collections::BTreeMap<i32, std::collections::VecDeque<Vec<i32>>> =
            std::collections::BTreeMap::new();
        for c in properties.iter() {
            for (_, s) in stacks.range_mut(..c[0]) {
                while !s.is_empty() && s[0][1] < c[1] {
                    s.pop_front();
                    count += 1;
                }
            }
            stacks
                .entry(c[0])
                .or_insert(std::collections::VecDeque::new())
                .push_back(c.to_vec());
        }
        count
    }
}
