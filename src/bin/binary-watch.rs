fn main() {
    println!("{:?}", Solution::read_binary_watch(1));
}

struct Solution;

impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        fn values(n: usize, ps: &[i32], limit: i32) -> Vec<i32> {
            let mut tmp: Vec<(i32, usize)> = vec![(0, 0)];
            for k in (1..=n).rev() {
                tmp = tmp
                    .iter()
                    .flat_map(|&(v, i)| ((i + 1)..=(ps.len() - k)).map(move |j| (v + ps[j], j)))
                    .collect();
            }
            tmp.into_iter()
                .map(|(v, _)| v)
                .filter(|x| *x < limit)
                .collect::<Vec<_>>()
        }
        if turned_on > 8 {
            return vec![];
        }
        let mut ans = vec![];
        let turned_on = turned_on as usize;
        for h in 0..=turned_on {
            let m = turned_on - h;
            if h > 3 || m > 5 {
                continue;
            }
            let hs = values(h, HOUR, 12);
            let ms = values(m, MINUTE, 60);
            for &hv in hs.iter() {
                for &mv in ms.iter() {
                    ans.push(format!("{}:{:02}", hv, mv));
                }
            }
        }
        ans
    }
}

const MINUTE: &[i32] = &[0, 1, 2, 4, 8, 16, 32];
const HOUR: &[i32] = &[0, 1, 2, 4, 8];
