fn main() {
    proconio::input! {
        N: usize,
        S: [String; N],
    }

    let mut row_counts = vec![0; N];
    let mut col_counts = vec![0; N];

    for i in 0..N {
        let s = S[i].chars().collect::<Vec<_>>();
        for j in 0..N {
            if s[j] == 'o' {
                row_counts[i] += 1;
                col_counts[j] += 1;
            }
        }
    }

    let mut count: i64 = 0;
    for i in 0..N {
        let s = S[i].chars().collect::<Vec<_>>();
        for j in 0..N {
            if s[j] == 'o' {
                count += (row_counts[i] - 1) * (col_counts[j] - 1);
            }
        }
    }

    println!("{}", count);
}
