use leetcode_rust::vec_vec_char;

fn main() {
    println!(
        "{}",
        Solution::maximal_square(vec_vec_char![
            ["1", "0", "1", "0", "0"],
            ["1", "0", "1", "1", "1"],
            ["1", "1", "1", "1", "1"],
            ["1", "0", "0", "1", "0"]
        ])
    );
    println!(
        "{}",
        Solution::maximal_square(vec_vec_char![["0", "1"], ["1", "0"]])
    );
    println!("{}", Solution::maximal_square(vec_vec_char![["0"]]));
    println!(
        "{}",
        Solution2::maximal_square(vec_vec_char![
            ["1", "0", "1", "0", "0"],
            ["1", "0", "1", "1", "1"],
            ["1", "1", "1", "1", "1"],
            ["1", "0", "0", "1", "0"]
        ])
    );
    println!(
        "{}",
        Solution2::maximal_square(vec_vec_char![["0", "1"], ["1", "0"]])
    );
    println!("{}", Solution2::maximal_square(vec_vec_char![["0"]]));
}

struct Solution;

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let mut max = 0;
        let column_num = matrix[0].len();
        let row_num = matrix.len();
        let mut heights = vec![0; column_num];
        let mut stack: Vec<usize> = vec![];
        for i in 0..row_num {
            let row = &matrix[i];
            for j in 0..column_num {
                heights[j] = if row[j] == '1' { heights[j] + 1 } else { 0 };
            }
            max = std::cmp::max(max, largest_square(&heights, &mut stack));
        }
        max * max
    }
}

fn largest_square(heights: &Vec<i32>, stack: &mut Vec<usize>) -> i32 {
    let mut res = 0;
    for i in 0..heights.len() {
        // 这里 <= 和 < 都可以
        while !stack.is_empty() && heights[i] <= heights[*stack.last().unwrap()] {
            let idx = stack.pop().unwrap();
            let width = if stack.is_empty() {
                i as i32
            } else {
                (i - stack.last().unwrap() - 1) as i32
            };
            res = std::cmp::max(res, heights[idx].min(width))
        }
        stack.push(i);
    }

    while !stack.is_empty() {
        let idx = stack.pop().unwrap();
        let width = if stack.is_empty() {
            heights.len() as i32
        } else {
            (heights.len() - stack.last().unwrap() - 1) as i32
        };
        res = std::cmp::max(res, heights[idx].min(width));
    }

    res
}

struct Solution2;

impl Solution2 {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut dp = vec![vec![0; m + 1]; n + 1];
        let mut ans = 0;
        for i in (0..n).rev() {
            for j in (0..m).rev() {
                if matrix[i][j] == '0' {
                    dp[i][j] = 0;
                    continue;
                }
                dp[i][j] = 1 + dp[i][j + 1].min(dp[i + 1][j]).min(dp[i + 1][j + 1]);
                ans = ans.max(dp[i][j]);
            }
        }
        ans * ans
    }
}
