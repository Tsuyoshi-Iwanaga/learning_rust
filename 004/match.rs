fn main() {
  let n: i32 = 10;
  
  match (n % 2) == 0 {
    true => println!("{} is even", n),
    false => println!("{} is odd", n),
  }

  match n {
    0 => println!("{} is 0", n),
    1..=9 => println!("{} is Less Than 10", n),
    _ => println!("{} is Grater Than 10", n),
  }

  match n {
    0 => println!("{} is 0", n),
    1|3|5|7|9 => println!("{} is Less Than 10 and odd", n),
    2|4|6|8|10 => println!("{} is Less Than 10 and even", n),
    _ => println!("{} is Greater Than 11", n),
  }
}