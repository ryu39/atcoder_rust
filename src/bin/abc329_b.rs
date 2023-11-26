fn main() {
    proconio::input! {
        n: usize,
        a: [i32;n]
    }

    let mut max = 0;
    let mut next_max = 0;

    for i in 0..n {
        if a[i] > max {
            next_max = max;
            max = a[i];
        } else if a[i] < max && a[i] > next_max {
            next_max = a[i];
        }
    }

    println!("{}", next_max);
}
