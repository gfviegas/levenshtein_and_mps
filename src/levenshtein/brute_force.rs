use std::cmp::min;

fn distance(x: &String, m: usize, y: &String, n: usize, evaluations: &mut i32) -> usize {
    *evaluations += 1;

    if m == 0 {
        return n;
    }

    if n == 0 {
        return m;
    }

    let cost = if x.chars().nth(m - 1) == y.chars().nth(n - 1) {
        0
    } else {
        1
    };

    let a = distance(x, m - 1, y, n, evaluations) + 1;
    let b = distance(x, m, y, n - 1, evaluations) + 1;
    let c = distance(x, m - 1, y, n - 1, evaluations) + cost;

    return min(min(a, b), c);
}

pub fn levenshtein_distance(x: &String, y: &String) -> (usize, i32) {
    let m = x.chars().count();
    let n = y.chars().count();
    let mut evaluations = 0;

    let d = distance(x, m, y, n, &mut evaluations);

    return (d, evaluations);
}
