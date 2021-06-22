use petgraph::unionfind::UnionFind;
use proconio::{fastout, input};

const MOD: u64 = 998_244_353;

#[fastout]
fn main() {
    input! {
        n: usize, k: u32,
        a: [[u32; n]; n],
    }
    let mut ufr = UnionFind::new(n);
    let mut ufc = UnionFind::new(n);
    for i in 0..n - 1 {
        for j in i + 1..n {
            if (0..n).all(|l| a[i][l] + a[j][l] <= k) {
                ufr.union(i, j);
            }
            if (0..n).all(|l| a[l][i] + a[l][j] <= k) {
                ufc.union(i, j);
            }
        }
    }
    let mut answer = 1;
    for uf in &[ufr, ufc] {
        let mut v = vec![0; n];
        for i in (0..n).map(|i| uf.find(i)) {
            v[i] += 1;
            answer = answer * v[i] % MOD;
        }
    }
    println!("{}", answer);
}
