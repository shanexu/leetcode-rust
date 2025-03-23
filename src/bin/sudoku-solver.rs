fn main() {
    let board_strs = vec![
        "53x x7x xxx",
        "6xx 195 xxx",
        "x98 xxx x6x",
        "8xx x6x xx3",
        "4xx 8x3 xx1",
        "7xx x2x xx6",
        "x6x xxx 28x",
        "xxx 419 xx5",
        "xxx x8x x79",
    ];
    let mut board = board_strs_to_board(board_strs);
    Solution::solve_sudoku(&mut board);
    println!("{:?}", board);
    let x = 0x01f0u16;
    println!("{}", x.count_zeros());
    println!("{:?}", MASKS);
}

struct Solution;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut rows = [0b111111111; 9];
        let mut columns = [0b111111111; 9];
        let mut squares = [0b111111111; 9];
        for i in 0..9 {
            for j in 0..9 {
                let v = board[i][j];
                if v != '.' {
                    let m = char_to_mask(v);
                    rows[i] = rows[i] & m;
                    columns[j] = columns[j] & m;
                    squares[i / 3 * 3 + j / 3] = squares[i / 3 * 3 + j / 3] & m;
                }
            }
        }
        solve(0, 0, board, &mut rows, &mut columns, &mut squares);
    }
}

const MASKS: [u16; 9] = [
    0b111111110,
    0b111111101,
    0b111111011,
    0b111110111,
    0b111101111,
    0b111011111,
    0b110111111,
    0b101111111,
    0b011111111,
];

const VALUES: [u16; 9] = [
    0b000000001,
    0b000000010,
    0b000000100,
    0b000001000,
    0b000010000,
    0b000100000,
    0b001000000,
    0b010000000,
    0b100000000,
];

fn char_to_mask(c: char) -> u16 {
    MASKS[(c as u8 - b'0' - 1) as usize]
}

fn solve(
    x: usize,
    y: usize,
    board: &mut Vec<Vec<char>>,
    rows: &mut [u16; 9],
    columns: &mut [u16; 9],
    squares: &mut [u16; 9],
) -> bool {
    let value = board[x][y];
    if value == '.' {
        let mut alt_values = rows[x] & columns[y] & squares[x / 3 * 3 + y / 3];
        if alt_values == 0 {
            return false;
        }
        let (x1, y1) = next(x, y);
        let mut k = 1;
        while alt_values > 0 {
            if alt_values & 1 == 1 {
                board[x][y] = (b'0' + k as u8) as char;
                if x == 8 && y == 8 {
                    return true;
                }
                rows[x] = rows[x] & MASKS[k - 1];
                columns[y] = columns[y] & MASKS[k - 1];
                squares[x / 3 * 3 + y / 3] = squares[x / 3 * 3 + y / 3] & MASKS[k - 1];
                if !solve(x1, y1, board, rows, columns, squares) {
                    rows[x] = rows[x] | VALUES[k - 1];
                    columns[y] = columns[y] | VALUES[k - 1];
                    squares[x / 3 * 3 + y / 3] = squares[x / 3 * 3 + y / 3] | VALUES[k - 1];
                    board[x][y] = '.';
                } else {
                    return true;
                }
            }
            k += 1;
            alt_values >>= 1;
        }
        return false;
    } else {
        if x == 8 && y == 8 {
            return true;
        } else {
            let (x1, y1) = next(x, y);
            return solve(x1, y1, board, rows, columns, squares);
        }
    }
}

fn next(x: usize, y: usize) -> (usize, usize) {
    if y + 1 == 9 {
        (x + 1, 0)
    } else {
        (x, y + 1)
    }
}

fn board_strs_to_board(strs: Vec<&str>) -> Vec<Vec<char>> {
    strs.into_iter()
        .map(|s| {
            s.replace(' ', "")
                .chars()
                .map(|c| if c == 'x' { '.' } else { c })
                .collect()
        })
        .collect()
}
