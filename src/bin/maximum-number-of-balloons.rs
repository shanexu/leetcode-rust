fn main() {
    assert_eq!(Solution::max_number_of_balloons("nlaebolko".to_string()), 1);
    assert_eq!(Solution::max_number_of_balloons("loonbalxballpoon".to_string()), 2);
    assert_eq!(Solution::max_number_of_balloons("leetcode".to_string()), 0);
    assert_eq!(Solution::max_number_of_balloons("balon".to_string()), 0);
    assert_eq!(Solution::max_number_of_balloons("balloon".to_string()), 1);
    assert_eq!(Solution::max_number_of_balloons("balloonballoon".to_string()), 2);
    assert_eq!(Solution::max_number_of_balloons("balloonballoonballoon".to_string()), 3);
    assert_eq!(Solution::max_number_of_balloons("balloonballoonballoonballoon".to_string()), 4);
    assert_eq!(Solution::max_number_of_balloons("balloonballoonballoonballoonballoon".to_string()), 5);
    assert_eq!(Solution::max_number_of_balloons("balloonballoonballoonballoonballoonballoon".to_string()), 6);
    assert_eq!(Solution::max_number_of_balloons("balloonballoonballoonballoonballoonballoonballoon".to_string()), 7);
    assert_eq!(Solution::max_number_of_balloons("balloonballoonballoonballoonballoonballoonballoonballoon".to_string()), 8);
    assert_eq!(Solution::max_number_of_balloons("balloonballoonballoonballoonballoonballoonballoonballoonballoon".to_string()), 9);
    assert_eq!(Solution::max_number_of_balloons("balloonballoonballoonballoonballoonballoonballoonballoonballoonballoon".to_string()), 10);
    assert_eq!(Solution::max_number_of_balloons("balloonballoonballoonballoonballoonballoonballoonballoonballoonballoonballoon".to_string()), 11);
    assert_eq!(Solution::max_number_of_balloons("balloonballoonballoonballoonballoonballoonballoonballoonballoonballoonballoonballoon".to_string()), 12);
}

/// balloon balon
struct Solution;

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let text = text.as_bytes();
        let mut freq = vec![0;5];
        for &b in text {
            match b {
                b'b' => freq[0] += 1,
                b'a' => freq[1] += 1,
                b'l' => freq[2] += 1,
                b'o' => freq[3] += 1,
                b'n' => freq[4] += 1,
                _ => (),
            }
        }
        freq[2] /= 2;
        freq[3] /= 2;

        *freq.iter().min().unwrap()
    }
}

