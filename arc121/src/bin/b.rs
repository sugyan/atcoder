use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut ac: [(i64, char); n * 2],
    }
    let mut rgb = vec![Vec::new(); 3];
    for &(a, c) in &ac {
        let i = match c {
            'R' => 0,
            'G' => 1,
            'B' => 2,
            _ => unreachable!(),
        };
        rgb[i].push(a);
    }
    if rgb.iter().all(|v| v.len() & 1 == 0) {
        println!("0");
        return;
    }
    rgb.iter_mut().for_each(|v| v.sort_unstable());
    let mut m = [[0; 3]; 3];
    for &(i, j) in &[(0, 1), (0, 2), (1, 2)] {
        let (mut ki, mut kj) = (0, 0);
        let mut min = match (rgb[i].is_empty(), rgb[j].is_empty()) {
            (true, false) => rgb[j][0],
            (false, true) => rgb[i][0],
            _ => (rgb[i][0] - rgb[j][0]).abs(),
        };
        while ki < rgb[i].len() && kj < rgb[j].len() {
            min = min.min((rgb[i][ki] - rgb[j][kj]).abs());
            if rgb[i][ki] < rgb[j][kj] {
                ki += 1;
            } else {
                kj += 1;
            }
        }
        m[i][j] = min;
        m[j][i] = min;
    }
    if let Some(e) = rgb.iter().position(|v| v.len() & 1 == 0) {
        let answer = m[(e + 1) % 3][(e + 2) % 3].min(m[e][(e + 1) % 3] + m[e][(e + 2) % 3]);
        println!("{}", answer);
    }
}
