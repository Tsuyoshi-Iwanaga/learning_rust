//排他制御(ミューテックス)
use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};

fn main() {
  let counter = Arc::new(Mutex::new(0));

  //first thread
  let counter1 = Arc::clone(&counter);
  let th1 = thread::spawn(move || {
    for _i in 1..11 {
      print!("+");
      let mut num = counter1.lock().unwrap();
      *num += 1;
      thread::sleep(Duration::from_millis(100));
    }
  });

  //second thread
  let counter2 = Arc::clone(&counter);
  let th2 = thread::spawn(move || {
    for _i in 1..11 {
      let mut num = counter2.lock().unwrap();
      *num += 1;
      print!("-");
      thread::sleep(Duration::from_millis(100));
    }
  });

  //wait for finish threads
  th1.join().unwrap();
  th2.join().unwrap();
  println!("\ncounter={:?}", counter);
}