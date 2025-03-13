use leetcode_rust::vec_vec_i32;

fn main() {
    println!(
        "{:?}",
        Solution::corp_flight_bookings(vec_vec_i32![[1, 2, 10], [2, 3, 20], [2, 5, 25]], 5)
    );
}

struct Solution;

impl Solution {
    pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut diff = vec![0; n];
        for b in bookings.iter() {
            let l = (b[0] - 1) as usize;
            let r = (b[1] - 1) as usize;
            diff[l] += b[2];
            if r < n - 1 {
                diff[r + 1] -= b[2];
            }
        }
        for i in 1..n {
            diff[i] += diff[i - 1];
        }
        diff
    }
}
