fn main() {
    println!(
        "{}",
        Solution::num_tile_possibilities("YYXBTSR".to_string())
    );
}

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        fn helper(
            i: usize,
            j: usize,
            freq: &Vec<usize>,
            remain: &Vec<usize>,
            memo: &mut HashMap<(usize, usize), i32>,
        ) -> i32 {
            if i == 0 {
                return 1;
            }
            if let Some(&v) = memo.get(&(i, j)) {
                return v;
            }
            let mut ans = 0;
            for k in i.checked_sub(remain[j + 1]).unwrap_or(0)..=freq[j].min(i) {
                ans += factorial(i) * helper(i - k, j + 1, freq, remain, memo)
                    / factorial(k)
                    / factorial(i - k);
            }
            memo.insert((i, j), ans);
            ans
        }
        let tiles = tiles.as_bytes();
        let mut freq = [0usize; 26];
        for &b in tiles {
            freq[(b - b'A') as usize] += 1;
        }
        let freq: Vec<usize> = freq
            .iter()
            .filter_map(|&x| if x != 0 { Some(x) } else { None })
            .collect();
        let mut remain = vec![0; freq.len()+1];
        for i in (0..freq.len()).rev() {
            remain[i] = remain[i + 1] + freq[i];
        }
        let mut memo = HashMap::new();
        let mut ans = 0;
        for i in (1..=tiles.len()).rev() {
            ans += helper(i, 0, &freq, &remain, &mut memo);
        }
        ans
    }
}

fn factorial(n: usize) -> i32 {
    FACTORIAL_NUMS[n]
}
const FACTORIAL_NUMS: &[i32] = &[1, 1, 2, 6, 24, 120, 720, 5040];
