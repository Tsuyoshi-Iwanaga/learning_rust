//列挙型 一連の値を定義した型、i32なども実は列挙型
//それぞれの値を列挙子という

enum DogKind {
  AKITAINU,
  SHIBAINU,
  RETRIEVER,
}


fn main() {
  //定義したもののインスタンスを作成していない列挙子があると警告が表示される
  let dog1 = DogKind::AKITAINU;
  let _dog2 = DogKind::SHIBAINU;
  let _dog3 = DogKind::RETRIEVER;

  match dog1 {
    DogKind::AKITAINU => println!("秋田犬"),
    DogKind::SHIBAINU => println!("柴犬"),
    DogKind::RETRIEVER => println!("レトリバー"),
  }
}

enum Command {
  Quit,//単一の値
  Move {x: i32, y: i32},//匿名構造体
  Write(String),//Stringオブジェクト
  SetColor(i32, i32, i32),//3つのi32値
}

enum Option<T> {
  Some(T),
  None,
}

enum Result<T, E> {
  Ok(T),
  Err(E),
}