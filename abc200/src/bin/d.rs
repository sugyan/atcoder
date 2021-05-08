use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }
    let a = a.iter().map(|m| (m % 200) as usize).collect::<Vec<_>>();
    if let Some((b, c)) = solve(&a) {
        println!("Yes");
        println!(
            "{} {}",
            b.len(),
            b.iter()
                .map(|i| (i + 1).to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
        println!(
            "{} {}",
            c.len(),
            c.iter()
                .map(|i| (i + 1).to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    } else {
        println!("No");
    }
}

fn solve(a: &[usize]) -> Option<(Vec<usize>, Vec<usize>)> {
    let mut v = vec![Vec::new(); 200];
    for (i, &a) in a.iter().enumerate() {
        if !v[a].is_empty() {
            return Some((vec![i], v[a].clone()));
        }
        v[a].push(i);
        for j in 0..200 {
            if !v[j].is_empty() && v[j].last() != Some(&i) {
                let mut w = v[j].clone();
                w.push(i);
                if !v[(j + a) % 200].is_empty() {
                    return Some((v[(j + a) % 200].clone(), w));
                }
                v[(j + a) % 200] = w;
            }
        }
    }
    None
}
