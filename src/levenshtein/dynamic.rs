use std::cmp::min;

pub fn levenshtein_distance(x: &String, y: &String) -> (usize, i32) {
    let m = x.chars().count();
    let n = y.chars().count();

    let mut evaluations: i32 = 0;
    let mut table = vec![vec![0; n + 1]; m + 1];

    // First column = just insert the character for each row value
    for i in 1..(m + 1) {
        table[i][0] = i; // (Case 1)
    }

    // First row = just insert the character for each column value
    for j in 0..(n + 1) {
        table[0][j] = j; // (Case 1)
    }

    // Bottom-up approach
    for i in 1..(m + 1) {
        for j in 1..(n + 1) {
            evaluations += 1;
            let cost = if x.chars().nth(i - 1) == y.chars().nth(j - 1) {
                0
            } else {
                1
            };

            let a = table[i - 1][j] + 1;
            let b = table[i][j - 1] + 1;
            let c = table[i - 1][j - 1] + cost;

            table[i][j] = min(min(a, b), c);
        }
    }

    return (table[m][n], evaluations);
}
