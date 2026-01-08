pub fn edit_distance(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();

    let n = a.len();
    let m = b.len();

    // D matrix: (n+1) x (m+1)
    let mut d = vec![vec![0; m + 1]; n + 1];

    // Base cases
    for i in 0..=n {
        d[i][0] = i;
    }
    for j in 0..=m {
        d[0][j] = j;
    }

    // Fill DP table
    for i in 1..=n {
        for j in 1..=m {
            let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };

            d[i][j] = std::cmp::min(
                std::cmp::min(
                    d[i - 1][j] + 1, 
                    d[i][j - 1] + 1     
                ),
                d[i - 1][j - 1] + cost  
            );
        }
    }

    d[n][m]
}
