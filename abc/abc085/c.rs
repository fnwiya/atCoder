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
    let n: i32 = read();
    let x: i32 = read();
    let mut ans10000 = -1;
    let mut ans5000 = -1;
    let mut ans1000 = -1;
    'found: for i in 0..n + 1 {
        for j in 0..n - i + 1 {
            let k = n - i - j;
            if i * 10000 + j * 5000 + k * 1000 == x {
                ans10000 = i;
                ans5000 = j;
                ans1000 = k;
                break 'found;
            }
        }
    }
    println!("{} {} {}", ans10000, ans5000, ans1000);
}