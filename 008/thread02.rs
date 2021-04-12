use std::thread;
use std::time::Duration;

fn main() {

  //clojure
  let func = |c| {
    for i in 1..10 {
      print!("{}:{}\n", c, i);
      thread::sleep(Duration::from_millis(100));
    }
  };

  //generate threads
  let th1 = thread::spawn(move || func("A"));
  let th2 = thread::spawn(move || func("B"));

  //wait for finish
  th1.join().unwrap();
  th2.join().unwrap();
  println!("all program finish!!");
}

