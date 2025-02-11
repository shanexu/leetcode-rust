fn main() {
    println!(
        "{}",
        Solution::remove_occurrences(
            "kpygkivtlqoockpygkivtlqoocssnextkqzjpycbylkaondsskpygkpygkivtlqoocssnextkqzjpkpygkivtlqoocssnextkqzjpycbylkaondsycbylkaondskivtlqoocssnextkqzjpycbylkaondssnextkqzjpycbylkaondshijzgaovndkjiiuwjtcpdpbkrfsi".to_string(),
            "kpygkivtlqoocssnextkqzjpycbylkaonds".to_string())
    );
    println!(
        "{}",
        Solution::remove_occurrences("daabcbaabcbc".to_string(), "abc".to_string())
    );
    println!(
        "{}",
        Solution::remove_occurrences("axxxxyyyyb".to_string(), "xy".to_string())
    );
}

struct Solution;

impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        let mut result = s;
        while let Some(pos) = result.find(&part) {
            result.replace_range(pos..pos + part.len(), "");
        }
        result
    }
}
