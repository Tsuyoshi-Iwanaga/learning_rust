//trait
struct Dog {
  name: String,
  age: i32,
}

struct Cat {
  name: String,
  age: i32,
}

trait Cry {
  fn cry(&self);
}

impl Cry for Dog {
  fn cry(&self) {
    println!("Baw Wow!");
  }
}

impl Cry for Cat {
  fn cry(&self) {
    println!("Nya Nya!");
  }
}

fn main() {
  let d = Dog {name: String::from("Pochi"), age: 6};
  let c = Cat {name: String::from("Tama"), age: 4};

  println!("{}({})", d.name, d.age);
  d.cry();

  println!("{}({})", c.name, c.age);
  c.cry();
}