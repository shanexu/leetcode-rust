fn main() {
    println!(
        "{}",
        Solution::min_swaps_couples(vec![0, 2, 4, 6, 7, 1, 3, 5])
    );
}

struct Solution;

impl Solution {
    pub fn min_swaps_couples(mut row: Vec<i32>) -> i32 {
        let n = row.len();
        let mut table = vec![0; n];
        for (i, &v) in row.iter().enumerate() {
            table[v as usize] = i;
        }
        let mut ans = 0;
        for i in (0..n).step_by(2) {
            if row[i] / 2 != row[i + 1] / 2 {
                let pair = if row[i] % 2 == 0 {
                    row[i] + 1
                } else {
                    row[i] - 1
                };
                let idx = table[pair as usize];
                row.swap(i + 1, idx);
                table[pair as usize] = i + 1;
                table[row[idx] as usize] = idx;
                ans += 1;
            }
        }
        ans
    }
}
