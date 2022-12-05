use std::cmp::min;

fn three_way_min(a: usize, b: usize, c: usize) -> usize {
    return min(a, min(b, c));
}

pub fn levenshtein_distance(x: &String, y: &String) -> (usize, i32) {
    let m = x.chars().count();
    let n = y.chars().count();
    
    let mut evaluations: i32 = 0;
    let mut table = vec![vec![0; n + 1]; m + 1];
    let mut cost: usize;

    // we can transform source prefixes into an empty string by dropping all characters
    for i in 1..(m + 1) {
        // evaluations += 1;
        table[i][0] = i; // (case 1)
    }
    
    // we can reach target prefixes from empty source prefix by inserting every character
    for j in 0..(n + 1) {
        // evaluations += 1;
        table[0][j] = j; // (case 1)
    }

    // fill the lookup table in a bottom-up manner
    for i in 1..(m + 1) {
        for j in 1..(n + 1) {
            evaluations += 1;
            cost = if x.chars().nth(i - 1) == y.chars().nth(j - 1) { 0 } else { 1 };

            let a = table[i - 1][j] + 1;
            let b = table[i][j - 1] + 1;
            let c = table[i - 1][j - 1] + cost;

            table[i][j] = three_way_min(a, b, c);
        }
    }

    return (table[m][n], evaluations);
}
