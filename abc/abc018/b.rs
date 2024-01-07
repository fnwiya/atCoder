use proconio::input;

fn main() {
    input! {
      s: String,
      n: usize,
      lr: [(usize, usize); n]
    }
    let mut ans = s;
    for (l, r) in lr {
        let pre = &ans[0..l - 1];
        let post = &ans[r..];
        let target = &ans[l - 1..r];
        ans = pre.to_owned() + &target.chars().rev().collect::<String>() + post;
    }
    println!("{}", ans)
}
