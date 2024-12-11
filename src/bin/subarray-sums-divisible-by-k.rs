fn main() {}

struct Solution;

impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut mods: Vec<i32> = vec![0; k as usize];
        let mut s = 0;

        for n in nums {
            s += n;
            let mut r = s % k;
            if r < 0 {
                r += k;
            }
            mods[r as usize] += 1;
        }

        mods.iter().fold(mods[0], |acc, c| acc + *c * (*c - 1) / 2)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cases() {
        let cases = vec![("case1", (vec![4, 5, 0, -2, -3, 1], 5), 7)];
        for (name, (nums, k), out) in cases {
            assert_eq!(Solution::subarrays_div_by_k(nums, k), out, "{name}");
        }
    }
}
