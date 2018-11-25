use std::cmp;
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
    let n: i64 = read();
    let h: i64 = read();
    let a: i64 = read();
    let b: i64 = read();
    let c: i64 = read();
    let d: i64 = read();
    let e: i64 = read();

    let mut ans = std::i64::MAX;

    for i in 0..n + 1 {
        let mut j = ((n - i) * e - h - b * i) / (d + e) + 1;
        if (n - i) * e - h - b * i < 0 {
            j = 0
        }
        ans = cmp::min(ans, a * i + c * j);
    }

    println!("{}", ans);
}
