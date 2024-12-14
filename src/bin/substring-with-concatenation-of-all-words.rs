//! # 为了描述方便定义几个变量
//! word_length: 单个单词的长度，根据题设，所有单词的长度相同
//! word_count: 单词的个数
//! total_length: word_length * word_count 所有单词拼在一起的长度
//! word_map: 单词频度表，例如单词为 ```["foo", "bar", "foo"]``` ， 那么 ```word_map``` 为
//! ```
//! {"foo": 2, "bar": 1}
//! ```
//!
//! # 一个没有成功的算法参考
//! 首先参考了
//! <https://www.geeksforgeeks.org/find-starting-indices-substrings-string-s-made-concatenating-words-listl/>
//! 但是很遗憾这个算法 ```Time Limit Exceeded``` 了。
//!
//! 这个算法可以大致描述为，以 ```total_length``` 为窗口大小，在整个字符串上以步长1来移动，在窗口上用 ```word_length```
//! 划分窗口为单词，然后丢入 ```tmp_word_map``` （此为```word_map```的克隆，每次移动窗口的时候生成） 中匹配，。
//! 如果找到单词，并且频度不为0，则对应单词频度自减1，否则退出循环。然后检查是否已经完成了word_count次匹配，如果是，
//! 则将窗口起始索引添加到返回结果集中。然后移动窗口，重复上面的步骤。
//!
//! # 在此算法上的改进
//!
//! ## 上面算法的问题
//! 上面的算法每次窗口移动一个字节，实际上会有很多重复计算。设窗口 W<sub>i</sub> 起始索引为```i```，另一个窗口 W<sub>i+word_length</sub> 起始索引为```i+word_length```
//! 这两个窗口实际重叠了 ```word_count-1``` 个单词，也就是说在计算 W<sub>i+word_length</sub> 时可以在 W<sub>i</sub> 的基础上多计算一个单词。
//!
//! ## 继续讨论在匹配单词上的优化，这里分三种情况
//! 1. 当前单词没有出现在 ```tmp_word_map```
//!    那么后续只要包含这个单词的都不可能匹配，所以窗口应该直接跳过这个单词，重新开始匹配
//! 2. 当前单词出现并频度不为0
//!    此时如果匹配了所有单词，那么将窗口起始索引添加到结果集，并将窗口移动 ```word_length``` 个位置，并调整 ```tmp_word_map```
//! 3. 当前单词（```current_word```）出现并频度为0
//!    此时应该将窗口移动到此次匹配，最早一次出现```current_word```之后，并在移动的过程中调整 ```tmp_word_map```。
//!
//! ## 继续讨论
//! 假设 ```f(i)``` 为起始位置为 ```i```，窗口移动步长为 ```word_length``` 的符合条件的结果集。
//! 那么 ```f(i) = f(i+word_length) + (substring(i,i+total_length)符合条件? : 1: 0)```
//! 所以只需要计算 `f(0) + f(1) + ... + f(word_length - 1)` 就是最终结果集了
fn main() {
    println!(
        "{:?}",
        Solution::find_substring(
            String::from("barfoothefoobarman"),
            vec![String::from("foo"), String::from("bar")]
        )
    );

    println!(
        "{:?}",
        Solution::find_substring(
            String::from("wordgoodgoodgoodbestword"),
            vec![
                String::from("word"),
                String::from("good"),
                String::from("best"),
                String::from("word")
            ],
        )
    );

    println!(
        "{:?}",
        Solution::find_substring(
            String::from("barfoofoobarthefoobarman"),
            vec![
                String::from("bar"),
                String::from("foo"),
                String::from("the"),
            ]
        )
    )
}

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let word_length = words[0].len();
        let word_count = words.len();
        let total_length = word_count * word_length;
        let bs = s.as_bytes();
        let s_len = bs.len();
        if total_length > bs.len() {
            return vec![];
        }
        let mut values = vec![];
        let mut word_map: HashMap<&[u8], usize> = HashMap::new();
        for x in words.iter() {
            let count = word_map.entry(x.as_bytes()).or_insert(0);
            *count += 1;
        }

        for i in 0..word_length {
            let mut tmp_word_map = word_map.clone();
            let mut j = i;
            let mut count = word_count;
            while j <= s_len - total_length {
                let mut k = j;
                while k <= s_len - word_length {
                    let word = &bs[k..k + word_length];
                    if let Some(v) = tmp_word_map.get_mut(word) {
                        if *v == 0 {
                            loop {
                                let first_word = &bs[j..j + word_length];
                                j += word_length;
                                if first_word == word {
                                    k += word_length;
                                    break;
                                } else {
                                    *(tmp_word_map.get_mut(first_word).unwrap()) += 1;
                                    count += 1;
                                }
                            }
                        } else {
                            *v -= 1;
                            count -= 1;
                            k += word_length;
                            if count == 0 {
                                values.push(j as i32);
                                let first_word = &bs[j..j + word_length];
                                *(tmp_word_map.get_mut(first_word).unwrap()) += 1;
                                count += 1;
                                j += word_length;
                            }
                        }
                    } else {
                        j = k + word_length;
                        tmp_word_map = word_map.clone();
                        count = word_count;
                        break;
                    }
                }
            }
        }
        values
    }
}
