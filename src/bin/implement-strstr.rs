fn main() {
    println!(
        "{}",
        Solution::str_str(String::from("helloworld"), String::from("hello"))
    );
}

struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        match haystack.find(&needle) {
            Some(i) => i as i32,
            None => -1,
        }
    }
}
