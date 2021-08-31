fn main() {
    let cases = [("PAYPALISHIRING", 3)];
    for (s, n) in cases.iter() {
        Solution::convert(String::from(*s), *n as i32);
    }
}


struct Solution {}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        // TODO
        String::from("")
    }
}
