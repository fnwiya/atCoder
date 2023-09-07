use proconio::input;

fn main() {
  const variation_size: usize = 1_000_001;
  let mut imos = [0; variation_size + 1];
  input! {
      n: usize,
  }
  for _ in 0..n {
    input! {
      a: usize,
      b: usize,
    }
    imos[a as usize] += 1;
    imos[b as usize + 1] -= 1;
  }
  for i in 0..variation_size {
    imos[i + 1] += imos[i];
  }
  let answer = imos.iter().max().unwrap();
  println!("{}", answer)
}