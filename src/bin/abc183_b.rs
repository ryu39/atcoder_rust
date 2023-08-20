// (sx, sy)と(gx, -gy)を通る直線、y = ax + bのa, bを求める。
// その後、y=0を代入し、x = -b / aを計算して求めるxを計算する。
fn main() {
    proconio::input! {
      sx: f64,
      sy: f64,
      gx: f64,
      gy: f64,
    }
    let a = (sy + gy) / (sx - gx);
    let b = sy - a * sx;
    let ax = -b / a;
    println!("{}", ax);
}
