fn main() {
    print_matrix(Solution::generate_matrix(20));
}

struct Solution {}

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut dx: i32 = 1;
        let mut dy: i32 = 0;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let m = n as usize;
        let mut matrix: Vec<Vec<i32>> = vec![vec![0; m]; m];
        for k in 1..=m * m {
            matrix[j as usize][i as usize] = k as i32;
            let ni = i + dx;
            let nj = j + dy;
            if ni == -1 || nj == -1 || ni == n || nj == n {
                (dx, dy) = dxy(dx, dy);
                i = i + dx;
                j = j + dy;
            } else if matrix[nj as usize][ni as usize] != 0 {
                (dx, dy) = dxy(dx, dy);
                i = i + dx;
                j = j + dy;
            } else {
                i = ni;
                j = nj;
            }
        }
        matrix
    }
}

#[inline]
fn dxy(dx: i32, dy: i32) -> (i32, i32) {
    match (dx, dy) {
        (1, 0) => (0, 1),
        (0, 1) => (-1, 0),
        (-1, 0) => (0, -1),
        (0, -1) => (1, 0),
        _ => (0, 0),
    }
}

fn print_matrix(matrix: Vec<Vec<i32>>) {
    for row in matrix {
        for k in row {
            print!("{:3} ", k);
        }
        println!();
    }
}
