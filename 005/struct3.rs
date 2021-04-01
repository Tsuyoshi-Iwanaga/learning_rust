//ジェネリック構造体
//任意の型を扱えるようにするための仕組み

struct Point<T, U> {
  x: T,
  y: U,
}

fn main() {
  let p1 = Point {x: 20, y: 10};
  let p2 = Point {x: 5, y: 6.0};

  println!("p1.x={}, p1.y={}", p1.x, p1.y);
  println!("p2.x={}, p2.y={}", p2.x, p2.y);
}