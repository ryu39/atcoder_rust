// t + k = x
// 2t + 4k = y
// t = (4x - y) / 2
// k = (y - 2x) / 2
fn main() {
    proconio::input! {
        x: i32,
        y: i32,
    }
    if y % 2 != 0 {
        println!("No");
    } else {
        let k = (y - 2 * x) / 2;
        if k >= 0 && k <= x {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
