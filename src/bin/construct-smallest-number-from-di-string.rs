fn main() {
    assert_eq!("1".to_string(), Solution::smallest_number("".to_string()));
    assert_eq!("12".to_string(), Solution::smallest_number("I".to_string()));
    assert_eq!(
        "123549876".to_string(),
        Solution::smallest_number("IIIDIDDD".to_string())
    );
    assert_eq!(
        "4321".to_string(),
        Solution::smallest_number("DDD".to_string())
    );
}

struct Solution;

impl Solution {
    pub fn smallest_number(pattern: String) -> String {
        fn help(index: usize, pattern: &[u8], visited: &mut Vec<bool>, path: &mut Vec<u8>) -> bool {
            if index == pattern.len() {
                return true;
            }
            let range = if pattern[index] == b'I' {
                (path[index] - b'1' + 1) as usize..9
            } else {
                0..(path[index] - b'1') as usize
            };
            for i in range {
                if !visited[i] {
                    path[index + 1] = i as u8 + b'1';
                    visited[i] = true;
                    if help(index + 1, pattern, visited, path) {
                        return true;
                    }
                    visited[i] = false;
                    path[index + 1] = 0;
                }
            }
            false
        }
        let pattern = pattern.as_bytes();
        let mut visited = vec![false; 9];
        let mut path = vec![0; pattern.len() + 1];
        for i in 0..9 {
            path[0] = i as u8 + b'1';
            visited[i] = true;
            if help(0, &pattern, &mut visited, &mut path) {
                return String::from_utf8(path).unwrap();
            }
            visited[i] = false;
            path[0] = 0;
        }
        unreachable!()
    }
}
