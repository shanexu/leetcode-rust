fn main() {
    println!("{}", Solution::to_lower_case("ABC".to_string()));
}

struct Solution;

impl Solution {
    pub fn to_lower_case(mut s: String) -> String {
        let bs = unsafe { s.as_bytes_mut() };
        for i in 0..bs.len() {
            if bs[i].is_ascii_uppercase() {
                bs[i] = bs[i].to_ascii_lowercase();
            }
        }
        s
    }
}
