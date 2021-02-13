fn use_variable() {
  let age = 10;
  let x = 5 + age;
  println!("age:{}", x);
  println!("----------");
}

fn not_work() {
  println!("Hello");
  ();
  println!("Rust");
  println!("----------");
}

fn use_int() {
  let _x = 18;
  const MAX_POINTS: u32 = 100_000;
  let _x16 = 0x12;
  let _x8 = 0o22;
  let _x2 = 0b010010;
  let _u = b'A';//10進数で65
  let _a = b';';//10進数で59
  let _n = 3_i16;
  println!("{}", MAX_POINTS);
  println!("----------");
}

fn use_float() {
  let x = 7.0;
  println!("{}", type_of(x));
  println!("----------");
}
//値の型を返す関数
fn type_of<T>(_: T) -> &'static str {
  std::any::type_name::<T>()
}

fn use_as() {
  let x: f32 = 7.0;
  let y: f64 = 0.3;
  println!("x*y:{}", x as f64 * y);//キャスト
  println!("----------");
}

fn use_str() {
  let msg: &str = "Hello World";
  println!("msg.len()={}", msg.len());
  println!("msg={}", msg);
  println!("msg={}", &msg[2..5]);
  println!("----------");
}

fn use_string() {
  let mut message = String::from("Hello");
  message = message + "Rust!";
  message.push_str("Study.");
  message.insert_str(0, "First");
  println!("{}", message);
  println!("----------");
}

fn use_type() {
  type Pop = u64;
  let jp_pop: Pop = 123_456_789;
  let es_pop: Pop = 46_723_749;

  let total = jp_pop + es_pop + 456789_u64;
  println!("total: {}", total);
  println!("----------");
}

fn main() {
  use_variable();
  not_work();
  use_int();
  use_float();
  use_as();
  use_str();
  use_string();
  use_type();
}
