fn main() {
    // println!("{:?}", Solution::get_row(0));
    // println!("{:?}", Solution::get_row(1));
    // println!("{:?}", Solution::get_row(2));
    println!("{:?}", Solution::get_row(3));
}

struct Solution;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let n = (row_index + 1) as usize;
        let mut row: Vec<i32> = vec![0; n];
        row[0] = 1;
        row[n - 1] = 1;
        for i in 1..=(n / 2) {
            let v = (row[i - 1] as i64 * (n - i) as i64 / i as i64) as i32;
            row[i] = v;
            row[n - i - 1] = v;
        }
        row
    }
}
