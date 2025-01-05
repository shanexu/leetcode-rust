fn main() {
    println!(
        "{}",
        Solution::shifting_letters(
            "abc".to_string(),
            vec![vec![0, 1, 0], vec![1, 2, 1], vec![0, 2, 1]]
        )
    );
}

struct Solution;

impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
        let s = s.as_bytes();
        let mut ans = vec![0; s.len()];

        let mut net_shifts: Vec<i32> = vec![0; s.len() + 1];
        for shift in shifts.iter() {
            let direction = if shift[2] == 1 { 1 } else { -1 };
            net_shifts[shift[0] as usize] += direction;
            net_shifts[(shift[1] + 1) as usize] -= direction;
        }
        for i in 1..net_shifts.len() {
            net_shifts[i] += net_shifts[i - 1];
        }
        for i in 0..s.len() {
            let shift = ((s[i] as i32 - b'a' as i32 + (net_shifts[i] % 26) + 26) % 26) as u8;
            ans[i] = b'a' + shift;
        }
        String::from_utf8(ans).unwrap()
    }
}
