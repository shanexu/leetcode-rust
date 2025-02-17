fn main() {
    assert!(!Solution::has_groups_size_x(vec![1]));
}

struct Solution;

impl Solution {
    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        let mut freq = vec![0; 10001];
        let mut x = 0;
        for num in deck {
            freq[num as usize] += 1;
        }
        for c in freq {
            if c == 0 {
                continue;
            }
            if x == 0 {
                x = c;
            } else {
                x = gcd(x, c);
            }
            if x == 1 {
                return false;
            }
        }
        true
    }
}

#[inline(always)]
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        (a, b) = (b, a % b)
    }
    a
}
