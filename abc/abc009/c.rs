use proconio::input;

fn main() {
    input! {
      n: usize,
      k: usize,
      s: String
    }

    let mut sorted_s: Vec<_> = s.chars().collect();
    sorted_s.sort();
    let mut ans = String::new();

    for i in 0..n {
        let remainded_chars = get_remaind_chars(&sorted_s, &ans);
        for remainded_char in &remainded_chars {
            let distance = calc_distance(&[ans.as_str(), &remainded_char.to_string()].concat(), &s);
            let unuseds = unuseds(&remainded_chars, remainded_char);
            let matches = set_matches(&s[i + 1..], &unuseds.iter().collect::<String>());
            if remainded_chars.len() - 1 - matches + distance <= k {
                ans.push(*remainded_char);
                break;
            }
        }
    }
    println!("{}", ans);
}

fn get_remaind_chars(sorted: &Vec<char>, used_chars: &str) -> Vec<char> {
    let mut remainded_chars = sorted.clone();
    for used in used_chars.chars() {
        remainded_chars.remove(remainded_chars.iter().position(|c| c == &used).unwrap());
    }
    remainded_chars
}

fn unuseds(usables: &Vec<char>, usable: &char) -> Vec<char> {
    let mut unuseds = usables.clone();
    unuseds.remove(usables.iter().position(|c| c == usable).unwrap());
    unuseds
}

fn calc_distance(a: &str, b: &str) -> usize {
    a.chars().zip(b.chars()).filter(|(a, b)| a != b).count()
}

fn set_matches(a: &str, b: &str) -> usize {
    let mut count = 0;
    for character in b'a'..=b'z' {
        let character = character as char;
        count += a
            .chars()
            .filter(|&c| c == character)
            .count()
            .min(b.chars().filter(|&c| c == character).count());
    }
    count
}
