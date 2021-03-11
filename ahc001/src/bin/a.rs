use proconio::input;

#[derive(Clone, Copy)]
struct Rect {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
}

impl Rect {
    fn size(&self) -> i32 {
        (self.x2 - self.x1) * (self.y2 - self.y1)
    }
    fn score(&self, r: i32) -> f32 {
        let s = self.size().min(r) as f32 / self.size().max(r) as f32;
        1.0 - (1.0 - s) * (1.0 - s)
    }
}

fn intersect(r1: &Rect, r2: &Rect) -> bool {
    r1.x2.min(r2.x2) > r1.x1.max(r2.x1) && r1.y2.min(r2.y2) > r1.y1.max(r2.y1)
}

fn main() {
    input! {
        n: usize,
        xyr: [(i32, i32, i32); n],
    };
    let mut v = Vec::new();
    for (x, y, r) in xyr {
        v.push((
            Rect {
                x1: x,
                y1: y,
                x2: x + 1,
                y2: y + 1,
            },
            r,
        ));
    }
    for i in 0..n * 1000 {
        let (mut rect, r) = v[i % n];
        match (i + i / n) % 4 {
            0 if rect.x1 >= 10 => rect.x1 -= 10,
            1 if rect.y1 >= 10 => rect.y1 -= 10,
            2 if rect.x2 <= 9990 => rect.x2 += 10,
            3 if rect.y2 <= 9990 => rect.y2 += 10,
            _ => {}
        }
        if rect.score(r) < v[i % n].0.score(r) {
            continue;
        }
        if !(0..n)
            .filter(|&j| j != i % n)
            .any(|j| intersect(&v[j].0, &rect))
        {
            v[i % n].0 = rect;
        }
    }
    for (rect, _) in v.iter() {
        println!("{} {} {} {}", rect.x1, rect.y1, rect.x2, rect.y2);
    }
}
