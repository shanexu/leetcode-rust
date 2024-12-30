fn main() {
    println!("{}", Solution::is_palindrome(String::from("a.a")));
    println!("{}", Solution::is_palindrome(String::from("aAc  ")));
    println!(
        "{}",
        Solution::is_palindrome(String::from("A man, a plan, a canal: Panama"))
    );
}

struct Solution {}

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let bs = s.as_bytes();
        if bs.len() == 0 {
            return true;
        }
        let mut left = (-1) as i32;
        let mut left_c: u8 = 0;
        let mut right = bs.len() as i32;
        let mut right_c: u8 = 0;
        loop {
            left = left + 1;
            while left < bs.len() as i32 {
                left_c = alphanumeric(bs[left as usize]);
                if left_c != 0 {
                    break;
                }
                left = left + 1;
            }
            right = right - 1;
            while right >= 0 {
                right_c = alphanumeric(bs[right as usize]);
                if right_c != 0 {
                    break;
                }
                right = right - 1;
            }
            if left >= right {
                return true;
            }
            // if left == (bs.len() as i32) && right == -1 {
            //     return true;
            // }
            if left_c != right_c {
                return false;
            }
        }
    }
}

#[inline]
fn alphanumeric(c: u8) -> u8 {
    if c >= b'A' && c <= b'Z' {
        return c - b'A' + b'a';
    }
    if c >= b'a' && c <= b'z' {
        return c;
    }
    if c >= b'0' && c <= b'9' {
        return c;
    }
    return 0;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert!(
            Solution::is_palindrome(String::from("A man, a plan, a canal: Panama")),
            "case1"
        );

        assert!(Solution::is_palindrome(String::from("A a")), "case2");
    }
}
