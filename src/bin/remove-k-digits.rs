fn main() {
    println!("{}", Solution::remove_kdigits("1234".to_string(), 2));
}

struct Solution;

impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let bs = num.as_bytes();
        let mut k = k as usize;
        if bs.len() == k {
            return "0".to_string();
        }
        let mut stack: Vec<u8> = vec![];
        for &x in bs {
            while k > 0 && !stack.is_empty() && stack[stack.len() - 1] > x {
                stack.pop();
                k -= 1;
            }
            stack.push(x);
        }
        for _ in 0..k {
            stack.pop();
        }
        let mut opt_j = None;
        for i in 0..stack.len() {
            if stack[i] == b'0' {
                opt_j = Some(i);
            } else {
                break;
            }
        }
        if let Some(j) = opt_j {
            if j == stack.len() - 1 {
                "0".to_string()
            } else {
                String::from_utf8_lossy(&stack[j + 1..]).to_string()
            }
        } else {
            String::from_utf8(stack).unwrap()
        }
    }
}
