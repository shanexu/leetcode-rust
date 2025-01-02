fn main() {
    println!(
        "{:?}",
        Solution::vowel_strings(
            vec![
                "bzmxvzjxfddcuznspdcbwiojiqf".to_string(),
                "mwguoaskvramwgiweogzulcinycosovozppl".to_string(),
                "uigevazgbrddbcsvrvnngfrvkhmqszjicpieahs".to_string(),
                "uivcdsboxnraqpokjzaayedf".to_string(),
                "yalc".to_string(),
                "bbhlbmpskgxmxosft".to_string(),
                "vigplemkoni".to_string(),
                "krdrlctodtmprpxwditvcps".to_string(),
                "gqjwokkskrb".to_string(),
                "bslxxpabivbvzkozzvdaykaatzrpe".to_string(),
                "qwhzcwkchluwdnqjwhabroyyxbtsrsxqjnfpadi".to_string(),
                "siqbezhkohmgbenbkikcxmvz".to_string(),
                "ddmaireeouzcvffkcohxus".to_string(),
                "kjzguljbwsxlrd".to_string(),
                "gqzuqcljvcpmoqlnrxvzqwoyas".to_string(),
                "vadguvpsubcwbfbaviedr".to_string(),
                "nxnorutztxfnpvmukpwuraen".to_string(),
                "imgvujjeygsiymdxp".to_string(),
                "rdzkpk".to_string(),
                "cuap".to_string(),
                "qcojjumwp".to_string(),
                "pyqzshwykhtyzdwzakjejqyxbganow".to_string(),
                "cvxuskhcloxykcu".to_string(),
                "ul".to_string(),
                "axzscbjajazvbxffrydajapweci".to_string()
            ],
            vec![vec![4, 4], vec![6, 17]]
        )
    );
}

struct Solution;

impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut prefix_sums = vec![0; words.len()];
        let mut count = 0;
        for (i, word) in words.iter().enumerate() {
            let bs = word.as_bytes();
            let b1 = bs[0];
            let b2 = bs[bs.len() - 1];
            if VOWEL.binary_search(&b1).is_ok() && VOWEL.binary_search(&b2).is_ok() {
                count += 1;
            }
            prefix_sums[i] = count;
        }
        let mut ans = vec![0; queries.len()];
        for (i, q) in queries.iter().enumerate() {
            let start = q[0] as usize;
            let end = q[1] as usize;
            if start == 0 {
                ans[i] = prefix_sums[end];
            } else {
                ans[i] = prefix_sums[end] - prefix_sums[start - 1];
            }
        }
        ans
    }
}

const VOWEL: &[u8] = &[b'a', b'e', b'i', b'o', b'u'];
