fn main() {
    assert_eq!(
        Solution::spiral_order(vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
        ]),
        vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
    );
    assert_eq!(
        Solution::spiral_order(vec![vec![1, 2, 3, 4]]),
        vec![1, 2, 3, 4]
    );
    assert_eq!(Solution::spiral_order(vec![vec![1]]), vec![1]);
}

struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut dir = 0;
        let mut i = 0;
        let mut j = 0;

        fn next(i: &mut usize, j: &mut usize, dir: &mut usize, m: usize, n: usize) {
            match *dir % 4 {
                0 => {
                    if *j < n - (*dir / 4 + 1) {
                        *j += 1;
                    } else {
                        *dir += 1;
                        next(i, j, dir, m, n);
                    }
                }
                1 => {
                    if *i < m - (*dir / 4 + 1) {
                        *i += 1;
                    } else {
                        *dir += 1;
                        next(i, j, dir, m, n);
                    }
                }
                2 => {
                    if *j > *dir / 4 {
                        *j -= 1;
                    } else {
                        *dir += 1;
                        next(i, j, dir, m, n);
                    }
                }
                3 => {
                    if *i > *dir / 4 + 1 {
                        *i -= 1;
                    } else {
                        *dir += 1;
                        next(i, j, dir, m, n);
                    }
                }
                _ => {}
            }
        }
        let mut res = Vec::with_capacity(m * n);
        res.push(matrix[i][j]);
        for _ in 1..m * n {
            next(&mut i, &mut j, &mut dir, m, n);
            res.push(matrix[i][j]);
        }
        res
    }
}
