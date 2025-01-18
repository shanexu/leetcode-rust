fn main() {}

struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut c0 = 1;

        // Traverse the array and mark the first cell of each row and column
        for i in 0..n {
            for j in 0..m {
                if matrix[i][j] == 0 {
                    // Mark the i-th row
                    matrix[i][0] = 0;

                    // Mark the j-th column
                    if j == 0 {
                        c0 = 0;
                    } else {
                        matrix[0][j] = 0;
                    }
                }
            }
        }

        // Traverse and mark the matrix from (1, 1) to (n - 1, m - 1)
        for i in 1..n {
            for j in 1..m {
                // Check for column & row
                if matrix[i][0] == 0 || matrix[0][j] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }

        // Mark the first row
        if matrix[0][0] == 0 {
            for j in 0..m {
                matrix[0][j] = 0;
            }
        }

        // Mark the first column
        if c0 == 0 {
            for i in 0..n {
                matrix[i][0] = 0;
            }
        }
    }
}
