fn main() {
    println!("{:?}", Solution::sort_array_by_parity(vec![3, 1, 2, 4]));
}

struct Solution;

impl Solution {
    pub fn sort_array_by_parity(a: Vec<i32>) -> Vec<i32> {
        let mut a = a;
        let mut i = 0;
        let mut j = a.len() - 1;
        while i < j {
            if a[i] % 2 == 0 {
                i += 1;
            } else if a[j] % 2 == 1 {
                j -= 1;
            } else {
                a.swap(i, j);
                i += 1;
                j -= 1;
            }
        }
        a
    }
}
