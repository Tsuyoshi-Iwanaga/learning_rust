//ジェネリック構造体
struct Point<T, U> {
  x: T,
  y: U,
}

trait PrintData {
  fn print(&self);
}

//TとUをstd::fmt::Displayに属している型に制限する
impl<T, U> PrintData for Point<T, U> where T: std::fmt::Display, U: std::fmt::Display {
  fn print(self: &Point<T, U>) {
    println!("({}, {})", self.x, self.y);
  }
}

fn main() {
  let p1 = Point {x: 20, y :10};
  let p2 = Point {x: 5, y: 6.0};

  p1.print();
  println!("p2.x: {}, p2.y: {}", p2.x, p2.y);
}