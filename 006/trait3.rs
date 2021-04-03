struct Point {
  x: i32,
  y: i32,
}

impl Clone for Point {
  fn clone(&self) -> Self {
    Point {
      //Rustのプリミティブ型は全てCloneとCopyトレイトを実装している
      x: self.x.clone(),
      y: self.y.clone(),
    }
  }
}

fn main() {
  let p = Point {x: 12, y: 25};
  print(p.clone()); //クローンを作って関数に渡す
  println!("({}, {})", p.x, p.y);//pの所有権はprint関数に移動せずエラーにはならない
}

fn print(p: Point) {
  println!("({}, {})", p.x, p.y);
}