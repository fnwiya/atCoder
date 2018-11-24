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
    let m: i32 = read();
    let mut a = -1;
    let mut b = -1;
    let mut c = -1;
    for i in 0..n + 1 {
        let j = 4 * n - 2 * i - m;
        let k = n - i - j;
        if j >= 0 && k >= 0 {
            a = i;
            b = j;
            c = k;
        }
    }
    println!("{} {} {}", a, b, c);
}
