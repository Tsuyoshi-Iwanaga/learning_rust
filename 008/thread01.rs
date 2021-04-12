use std::thread;
use std::time::Duration;

fn printch(c: char) {
  println!("thread {} start!", c);
  for i in 1..10 {
    print!("{}:{}\n", c, i);
    thread::sleep(Duration::from_millis(100));
  }
  println!("thread {} finish!", c);
}

fn main() {
  let th1 = thread::spawn(|| printch('A'));
  let th2 = thread::spawn(|| printch('B'));
  let th3 = thread::spawn(|| printch('C'));

  //wait for thread finish
  th1.join().unwrap();
  th2.join().unwrap();
  th3.join().unwrap();

  println!("all finished!!");
}
