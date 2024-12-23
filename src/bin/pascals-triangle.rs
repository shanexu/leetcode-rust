fn main() {
    println!("{:?}", Solution::generate(4));
}

struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let n = num_rows as usize - 1;
        let mut ans = vec![vec![1]];
        for i in 1..=n {
            let prev_row = &ans[i - 1];
            let mut row = Vec::with_capacity(i + 1);
            row.push(1);
            for j in 1..i {
                row.push(prev_row[j - 1] + prev_row[j]);
            }
            row.push(1);
            ans.push(row);
        }
        ans
    }
}
