fn main() {
    let matrix = vec![
        vec!['1', '0', '1', '0', '0'],
        vec!['1', '0', '1', '1', '1'],
        vec!['1', '1', '1', '1', '1'],
        vec!['1', '0', '0', '1', '0'],
    ];
    let res = Solution::maximal_rectangle(matrix);
    println!("res = {}", res);
}

struct Solution;

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
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
            max = std::cmp::max(max, largest_rectangle_area(&heights, &mut stack));
        }
        max
    }
}

fn largest_rectangle_area(heights: &Vec<i32>, stack: &mut Vec<usize>) -> i32 {
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
            res = std::cmp::max(res, heights[idx] * width)
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
        res = std::cmp::max(res, heights[idx] * width);
    }

    res
}
