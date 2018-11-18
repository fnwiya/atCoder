use std::io::*;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

fn main() {
    const TARGET: char = '#';
    let h: usize = read();
    let w: usize = read();
    let board: Vec<Vec<char>> = (0..h).map(|_| read::<String>().chars().collect()).collect();
    let mut flg = true;
    for j in 0..h {
        for k in 0..w {
            if board[j][k] == TARGET {
                let mut f = false;
                if 0 < j && board[j - 1][k] == TARGET {
                    f = true;
                }
                if 0 < k && board[j][k - 1] == TARGET {
                    f = true;
                }
                if j < h - 1 && board[j + 1][k] == TARGET {
                    f = true;
                }
                if k < w - 1 && board[j][k + 1] == TARGET {
                    f = true;
                }
                if !f {
                    flg = false;
                    break;
                }
            }
        }
    }
    println!("{}", if flg { "Yes" } else { "No" })
}
