//構造体は中に複数をフィールドをもつ
//CやC++の構造体とは異なりメソッドを持つことができるためクラスに近い
struct Member {
  id: String,
  name: String,
  age: i32,
}

fn main() {
  let m = Member {
    id: String::from("A0101"),
    name: String::from("Pochi"),
    age: 12,
  };

  println!("id: {}", m.id);
  println!("name: {}", m.name);
  println!("age: {}", m.age);

  //mutをつけると中身を変更できる
  let mut m2 = Member {
    id: String::from("A0102"),
    name: String::from("Hana"),
    age: 8,
  };

  m2.name = String::from("Tarou");
  m2.age = 10;
 
  println!("id: {}", m2.id);
  println!("name: {}", m2.name);
  println!("age: {}", m2.age);
}