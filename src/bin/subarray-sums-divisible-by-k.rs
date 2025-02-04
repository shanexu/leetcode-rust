fn main() {
    println!(
        "{}",
        Solution::subarrays_div_by_k(vec![4, 5, 0, -2, -3, 1], 5)
    );
    println!("{}", Solution::subarrays_div_by_k(vec![5], 9));
}

struct Solution;

impl Solution {
    // 先从0到i计算和，计算余数k，并对余数计数存入到mods[k]，mods[k]表示，余数为k的有几个。
    // 假设余数为k，mods[k]为c，Si表示从0到i的和，Sj表示从0到j的和，他们的余数都是k。
    // 那么 Sj-Si 能够被k整除，也就是i+1到j的和能够被k整除。那么这样的ij对有几个？
    // 其实就是c中取两个 c * (c-1) / 2
    // 其中 k=0 的时候是特殊情况，还要加上mods[0]
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
