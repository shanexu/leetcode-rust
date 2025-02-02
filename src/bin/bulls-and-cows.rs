fn main() {}

struct Solution;

impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let secret = secret.as_bytes();
        let guess = guess.as_bytes();
        let mut secret_freq = vec![0; 10];
        let mut guess_freq = vec![0; 10];
        let mut bulls = 0;
        let n = secret.len();
        for i in 0..n {
            let sb = secret[i];
            secret_freq[(sb - b'0') as usize] += 1;
            let gb = guess[i];
            guess_freq[(gb - b'0') as usize] += 1;
            if sb == gb {
                bulls += 1;
            }
        }
        let mut cows = 0;
        for i in 0..10 {
            cows += secret_freq[i].min(guess_freq[i]);
        }
        cows -= bulls;
        format!("{}A{}B", bulls, cows)
    }
}
