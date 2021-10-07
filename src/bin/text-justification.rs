fn main() {
    assert_eq!(
        Solution::full_justify(
            vec![
                String::from("This"),
                String::from("is"),
                String::from("an"),
                String::from("example"),
                String::from("of"),
                String::from("text"),
                String::from("justification.")
            ],
            16
        ),
        vec![
            String::from("This    is    an"),
            String::from("example  of text"),
            String::from("justification.  ")
        ]
    );
    assert_eq!(
        Solution::full_justify(
            vec![
                String::from("What"),
                String::from("must"),
                String::from("be"),
                String::from("acknowledgment"),
                String::from("shall"),
                String::from("be")
            ],
            16
        ),
        vec![
            String::from("What   must   be"),
            String::from("acknowledgment  "),
            String::from("shall be        ")
        ]
    )
}

struct Solution;

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let max_width = max_width as usize;
        let mut lines: Vec<(Vec<&String>, usize)> = vec![];
        for word in words.iter() {
            if let Some(v) = lines.last_mut() {
                if v.1 + 1 + word.len() > max_width {
                    lines.push((vec![word], word.len()));
                } else {
                    v.0.push(word);
                    v.1 += 1;
                    v.1 += word.len();
                }
            } else {
                lines.push((vec![word], word.len()));
            }
        }
        let mut results = Vec::with_capacity(lines.len());
        for (i, (ws, s)) in lines.iter().enumerate() {
            let mut str = String::new();
            if ws.len() == 1 {
                str = str + ws[0];
                append_space(&mut str, max_width - s);
            } else if i == lines.len() - 1 {
                let gap = ws.len() - 1;
                for (j, w) in ws.iter().enumerate() {
                    str = str + w;
                    if j != gap {
                        append_space(&mut str, 1);
                    }
                }
                append_space(&mut str, max_width - s);
            } else {
                let gap = ws.len() - 1;
                let total_space = max_width - s + gap;
                let avg_gap_width = total_space / gap;
                let remaining_space = total_space - avg_gap_width * gap;
                for (j, w) in ws.iter().enumerate() {
                    str = str + w;
                    if j != gap {
                        if j < remaining_space {
                            append_space(&mut str, 1);
                        }
                        append_space(&mut str, avg_gap_width);
                    }
                }
            }
            results.push(str);
        }
        results
    }
}

fn append_space(str: &mut String, n: usize) {
    for _ in 0..n {
        str.push(' ');
    }
}
