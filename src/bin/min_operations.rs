use leetcode_rust::vec_vec_i32;

fn main() {
    println!(
        "{}",
        Solution::min_operations(vec_vec_i32![[2, 4], [6, 8]], 2)
    );
}

struct Solution;

impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut nums = Vec::with_capacity(m * n);
        for i in 0..m {
            for j in 0..n {
                nums.push(grid[i][j]);
            }
        }
        nums.sort_unstable();
        let mut s = 0;
        let num0 = nums[0];
        let mn = m * n;
        for i in 1..mn {
            let num = nums[i];
            let d = num - num0;
            if d % x != 0 {
                return -1;
            }
            s += d;
        }
        let mut sm = s;
        for i in 1..mn {
            let d = nums[i] - nums[i - 1];
            if d != 0 {
            s = s - (mn - i) as i32 * d + i as i32 * d;
            sm = sm.min(s);
            }
            
        }
        sm / x
    }
}
