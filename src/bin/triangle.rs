fn main() {
    println!(
        "{}",
        Solution::minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]])
    );
}

struct Solution;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut prev: Vec<i32> = vec![triangle[0][0]];
        let mut current: Vec<i32> = vec![];
        for i in 1..triangle.len() {
            for j in 0..=i {
                let mut m = i32::MAX;
                if j > 0 {
                    m = m.min(prev[j - 1]);
                }
                if j < i {
                    m = m.min(prev[j]);
                }
                current.push(m + triangle[i][j]);
            }
            std::mem::swap(&mut prev, &mut current);
            current.clear();
        }
        prev.iter().min().unwrap().clone()
    }
}
