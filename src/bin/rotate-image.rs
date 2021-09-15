fn main() {
    let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    Solution::rotate(&mut matrix);
    println!("{:?}", matrix);
}

struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                matrix[j][n - i - 1] =
                    combine_old_new_value(matrix[j][n - i - 1], get_old_value(matrix[i][j]));
            }
        }
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                matrix[i][j] = get_new_value(matrix[i][j]);
            }
        }
    }
}

#[inline]
fn get_old_value(x: i32) -> i32 {
    ((x as u32) & 0x0000ffff) as i16 as i32
}

#[inline]
fn combine_old_new_value(x: i32, y: i32) -> i32 {
    (((y as u32 & 0x0000ffff) << 16) | (x as u32 & 0x0000ffff)) as i32
}

#[inline]
fn get_new_value(x: i32) -> i32 {
    ((x as u32) >> 16) as i16 as i32
}
