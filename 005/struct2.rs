//構造体を入れ子にすることもできる

struct Point {
  x: i32,
  y: i32,
}

struct Circle {
  center: Point,//入れ子
  radius: i32,
}

fn main() {
  let c = Circle {
    center: Point { x: 10, y: 20},
    radius: 25,
  };

  println!("(x,y) = ({}, {})", c.center.x, c.center.y);
  println!("half = {}", c.radius);
}