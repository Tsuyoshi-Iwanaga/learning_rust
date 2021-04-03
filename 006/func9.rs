//Option
fn main() {
  let a = vec![1, 3, 5, 7, 9];
  let n = 5;

  match getvalue(&a, n) {
    Some(result) => println!("Value:{}", result),
    None => println!("{} is None", n),
  }
}

fn getvalue(a: &Vec<i32>, n: i32) -> Option<i32> {
  for x in a.iter() {
    if x == &n {
      return Some(n);
    }
  }
  None
}