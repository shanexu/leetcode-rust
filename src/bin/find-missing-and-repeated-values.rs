fn main() {
    println!(
        "{:?}",
        Solution::find_missing_and_repeated_values(vec![
            vec![1, 2, 2],
            vec![4, 5, 6],
            vec![7, 8, 9]
        ])
    );
}

/// 空间复杂度 O(n2)
struct Solution;

impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid.len();
        let bl = (n * n + 7) >> 3;
        let mut bit_map: Vec<u8> = vec![0; bl];
        let mut missing = ((1 + n * n) * n * n / 2) as i32;
        let mut repeated = 0;
        for i in 0..n {
            for j in 0..n {
                let t = grid[i][j];
                let bit_idx = (t - 1) as usize >> 3;
                let bit_pos = (t - 1) as usize & 7;
                if bit_map[bit_idx] & (1 << bit_pos) == 0 {
                    bit_map[bit_idx] |= 1 << bit_pos;
                    missing -= t
                } else {
                    repeated = t;
                }
            }
        }
        vec![repeated, missing]
    }
}
