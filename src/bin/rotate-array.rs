fn main() {
    let mut vec: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    Solution::rotate(&mut vec, 2);
    println!("{:?}", vec);
}

struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();
        let k = k as usize % n;
        let m = gcd(n, k);
        for i in 0..m {
            let mut j = (i + k) % n;
            while j != i {
                nums.swap(j, i);
                j = (j + k) % n;
            }
        }
    }
}

#[inline(always)]
fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
