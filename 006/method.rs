//構造体を定義
struct Point {
  x: i32,
  y: i32,
}

impl Point {
  //output
  fn print(&self) {
    println!("({}, {})", self.x, self.y);
  }
  //multiple value
  fn mult(&self, n: i32) -> (i32, i32) {
    let a = self.x * n;
    let b = self.y * n;
    (a, b)
  }
}

fn main() {
  let p = Point {x: 10, y: 20};
  p.print();
  println!("{:?}", p.mult(3));
}