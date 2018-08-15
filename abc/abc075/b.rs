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
    const MINE: char = '#';
    let h: u32 = read();
    let w: u32 = read();
    let dy = [0, 1, 0, -1, 1, 1, -1, -1];
    let dx = [1, 0, -1, 0, 1, -1, -1, 1];
    let mut board: Vec<Vec<char>> = (0..h).map(|_| read::<String>().chars().collect()).collect();
    for y in 0..h {
        for x in 0..w {
            if board[y as usize][x as usize] == MINE {
                continue
            }
            let mut num: u32 = 0;
            for i in 0..8 {
                let ny = (y as i32) + dy[i as usize];
                let nx = (x as i32) + dx[i as usize];
                if(nx >= 0 && nx < w as i32 && ny >= 0 && ny < h as i32 && board[ny as usize][nx as usize] == MINE) {
                    num += 1;
                }

            }
            board[y as usize][x as usize] = std::char::from_digit(num as u32, 10).unwrap();
        }
    }
    for y in 0..h {
        for x in 0..w {
            print!("{}", board[y as usize][x as usize]);
        }
        println!("");
    }
}