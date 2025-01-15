fn main() {}

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

