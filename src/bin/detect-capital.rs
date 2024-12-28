fn main() {

}

struct Solution;

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let is_all_uppercase = word.chars().all(|c| c.is_ascii_uppercase());
        if is_all_uppercase {
            return true;
        }
        let is_all_lowercase = word.chars().all(|c| c.is_ascii_lowercase());
        if is_all_lowercase {
            return true;
        }
        let is_first_uppercase_rest_lowercase = {
            let mut chars = word.chars();
            if let Some(first) = chars.next() {
                first.is_ascii_uppercase() && chars.all(|c| c.is_ascii_lowercase())
            } else {
                false
            }
        };
        is_first_uppercase_rest_lowercase
    }
}