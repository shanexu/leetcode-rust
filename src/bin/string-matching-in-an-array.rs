fn main() {
    println!(
        "{:?}",
        Solution::string_matching(vec![
            "ehvj".to_string(),
            "ehvjw".to_string(),
            "mmgeue".to_string(),
            "ehvjwy".to_string(),
            "hdqtlhtwhx".to_string(),
            "hdqtlhtwhxo".to_string(),
            "wkkxjsns".to_string(),
            "kohdqtlhtwhxfw".to_string(),
            "acftf".to_string(),
            "kohdqtlhtwhxodp".to_string(),
            "cwpeiowwms".to_string(),
            "ehvjk".to_string(),
            "mkdmsmxb".to_string(),
            "mehvjks".to_string(),
            "mxdoz".to_string(),
            "xnacftf".to_string(),
            "mgksgencwhk".to_string(),
            "hacftf".to_string(),
            "jdofko".to_string(),
            "mcwpeiowwms".to_string(),
            "x".to_string(),
            "pommgeuefd".to_string(),
            "kenptsaoyr".to_string(),
            "bmgksgencwhk".to_string(),
            "pcmgvojskh".to_string(),
            "xnacftfx".to_string(),
            "fpnpzvmckle".to_string(),
            "pjdofkone".to_string(),
            "ssheyptxddttxjm".to_string(),
            "xxnacftfxp".to_string(),
        ])
    );
}

struct Solution;

impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        let n = words.len();
        let mut visited = vec![false; n];
        let mut ans = vec![];
        for i in 0..n {
            if visited[i] {
                continue;
            }
            for j in (i + 1)..n {
                if words[i].contains(&words[j]) {
                    if !visited[j] {
                        ans.push(words[j].clone());
                        visited[j] = true;
                    }
                } else if words[j].contains(&words[i]) {
                    ans.push(words[i].clone());
                    break;
                }
            }
        }
        ans
    }
}
