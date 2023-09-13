use proconio::input;

fn main() {
    input! {
      n: usize,
      m: usize,
      ab: [(usize, usize); m]
    }
    let max = n * m;
    let mut frends_map = vec![vec![max; n]; n];
    for i in 0..n {
        frends_map[i][i] = 0;
    }
    for (a, b) in ab {
        frends_map[a - 1][b - 1] = 1;
        frends_map[b - 1][a - 1] = 1;
    }
    for j in 0..n {
        for k in 0..n {
            for l in 0..n {
                frends_map[k][l] =
                    std::cmp::min(frends_map[k][l], frends_map[k][j] + frends_map[j][l]);
            }
        }
    }
    for i in 0..n {
        let answer = frends_map[i].iter().filter(|&&v| v == 2).count();
        println!("{}", answer);
    }
}
