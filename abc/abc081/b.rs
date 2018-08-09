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
    let N : usize = read();

    let mut ans = 1000000001;
    for i in 0..N {
        let mut a: i64 = read();
        let mut tmp = 0;
        while a%2 == 0 {
            tmp += 1;
            a = a/2;
        }

        ans = std::cmp::min(ans, tmp);
    }

    println!("{}", ans);
}