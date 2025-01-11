fn main() {
    println!("{}", Solution::can_construct("annabelle".to_string(), 2));
    println!("{}", Solution::can_construct("aa".to_string(), 0));
}

/// 统计所有出现字符的个数之后，可以做一个推演。首先组合的下限是什么？
/// 奇数频度的字符个数。因为奇数频度只能作为回文数的核心。
/// 但是还有一种特殊情况，就是没有奇数频度的字符，那么最小组合个数就是1。
/// 那么组合的上限是什么？
/// 我们把其余字符成对（两个一组）的放在一起看，他们可以选在放在奇数字符的两边，
/// 也可以自己组成一个回文字符串，也可以分别拆开成两个独立字符串，也就是说
/// 一对字符串可以出现0, 1, 2三种情况，那么上限也就出来了。
struct Solution;

impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        let mut freq = vec![0; 26];
        let s = s.as_bytes();
        for &b in s {
            freq[(b - b'a') as usize] += 1;
        }
        let mut p = 0; // 奇数
        let mut q = 0; // 偶数
        for f in freq {
            p += f & 1;
            q += f >> 1;
        }
        k >= p && k <= p + 2 * q
    }
}
