fn main() {
    proconio::input! {
        x1: i32,
        x2: i32,
        x3: i32,
        x4: i32,
        x5: i32,
    }
    let x_list = [x1, x2, x3, x4, x5];
    for (i, x) in x_list.iter().enumerate() {
        if *x == 0 {
            println!("{}", i + 1);
            break;
        }
    }
}
