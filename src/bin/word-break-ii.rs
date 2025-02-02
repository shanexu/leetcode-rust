fn main() {
    assert_eq!(
        Solution::word_break(
            "catsanddog".to_string(),
            vec![
                "cat".to_string(),
                "cats".to_string(),
                "and".to_string(),
                "sand".to_string(),
                "dog".to_string()
            ]
        ),
        vec!["cat sand dog", "cats and dog"]
    );
    assert_eq!(
        Solution::word_break(
            "pineapplepenapple".to_string(),
            vec![
                "apple".to_string(),
                "pen".to_string(),
                "applepen".to_string(),
                "pine".to_string(),
                "pineapple".to_string()
            ]
        ),
        vec![
            "pine apple pen apple",
            "pine applepen apple",
            "pineapple pen apple"
        ]
    );
}

struct Solution;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        fn helper(
            i: usize,
            s: &str,
            dict: &[String],
            parts: &mut Vec<String>,
            ans: &mut Vec<String>,
        ) {
            if i == s.len() {
                ans.push(parts.join(" "));
            }
            let ts = &s[i..];
            for segment in dict {
                if ts.starts_with(segment) {
                    parts.push(segment.to_string());
                    helper(i + segment.len(), s, dict, parts, ans);
                    parts.pop();
                }
            }
        }
        let mut ans: Vec<String> = vec![];
        helper(0, &s, &word_dict, &mut vec![], &mut ans);
        ans
    }
}
