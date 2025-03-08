fn main() {
    assert_eq!(3, Solution::minimum_recolors("WBBWWBBWBW".to_string(), 7));
    assert_eq!(0, Solution::minimum_recolors("WBWBBBW".to_string(), 2));
}

struct Solution;

impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let blocks = blocks.as_bytes();
        let n = blocks.len();
        let k = k as usize;
        let mut wb = 0;
        for i in 0..k {
            if blocks[i] == b'W' {
                wb += 1;
            }
        }
        let mut ans = wb;
        for i in k..n {
            if ans == 0 {
                return 0
            }
            if blocks[i - k] == b'W' {
                wb -= 1;
            }
            if blocks[i] == b'W' {
                wb += 1;
            }
            ans = ans.min(wb);
        }
        ans
    }
}
