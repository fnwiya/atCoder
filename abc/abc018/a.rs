use proconio::input;

fn main() {
    input! {
      taro: usize,
      ziro: usize,
      saburo: usize
    }
    let original = [taro, ziro, saburo];
    let mut sorted = [taro, ziro, saburo];
    sorted.sort();
    sorted.reverse();
    for n in original {
        let ans = sorted.iter().position(|&x| x == n).unwrap() + 1;
        println!("{}", ans);
    }
}
