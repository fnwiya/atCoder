use proconio::input;

fn main() {
    input! {
      n: usize,
      m: usize,
      lrs: [(usize, usize, isize); n]
    }
    let mut imos = vec![0; m + 2];
    let mut sum = 0;
    for (l, r, s) in lrs {
        sum += s;
        imos[l] += s;
        imos[r + 1] -= s;
    }
    for i in 0..(m + 1) {
        imos[i + 1] += imos[i];
    }
    let min = imos[1..=m].iter().min().unwrap();
    let ans = sum - min;
    println!("{}", ans);
}
