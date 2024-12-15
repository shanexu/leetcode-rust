use leetcode_rust::utils::measure_time;

fn main() {
    println!("Solution: {:?}", measure_time(||Solution::unique_paths(4, 3)));
    println!("Solution2: {:?}", measure_time(||Solution2::unique_paths(4, 3)));
    println!("Solution3: {:?}", measure_time(||Solution3::unique_paths(4, 3)));

    println!("Solution: {:?}", measure_time(||Solution::unique_paths(51, 9)));
    println!("Solution2: {:?}", measure_time(||Solution2::unique_paths(51, 9)));
    println!("Solution3: {:?}", measure_time(||Solution3::unique_paths(51, 9)));

    println!("Solution: {:?}", measure_time(||Solution::unique_paths(16, 16)));
    println!("Solution2: {:?}", measure_time(||Solution2::unique_paths(16, 16)));
    println!("Solution3: {:?}", measure_time(||Solution3::unique_paths(16, 16)));
}

/// 直接通过数学排列组合求解，注意溢出问题
struct Solution;
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut mm = (m - 1) as u128;
        let mut nn = (n - 1) as u128;
        if m == 0 || n == 0 {
            return 1;
        }
        if mm > nn {
            std::mem::swap(&mut mm, &mut nn);
        }
        let s = mm + nn;
        ((s - mm + 1..=s).product::<u128>() / (1..=mm).product::<u128>()) as i32
    }
}

/// 递归求解，会超时
struct Solution2;
impl Solution2 {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        #[inline]
        fn helper(m: i32, n: i32, count: &mut i32) {
            if m == 1 || n == 1 {
                *count += 1;
                return;
            }
            helper(m - 1, n, count);
            helper(m, n - 1, count);
        }
        let mut count: i32 = 0;
        helper(m, n, &mut count);
        count
    }
}

/// 动态规划，比直接求解略慢
struct Solution3;
impl Solution3 {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        if m == 1 || n == 1 {
            return 1;
        }
        let m = m as usize;
        let n = n as usize;
        let mut memo: Vec<Vec<i32>> = vec![vec![1;n];m];
        for i in (0..m-1).rev() {
            for j in (0..n-1).rev() {
                memo[i][j] = memo[i+1][j] + memo[i][j+1];
            }
        }
        memo[0][0]
    }
}

