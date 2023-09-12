use proconio::input;

fn main() {
    input! {
      n: usize,
      k: usize,
      t: [[usize; k]; n]
    }
    let found = dfs(0, 0, n, k, &t);
    println!("{}", if found { "Found" } else { "Nothing" });
}

fn dfs(num_q: usize, value: usize, n: usize, k: usize, t: &Vec<Vec<usize>>) -> bool {
    if num_q == n {
        return value == 0;
    }
    (0..k).any(|i| dfs(num_q + 1, value ^ t[num_q][i], n, k, t))
}
