use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
  //generate channel
  let (tx, rx) = mpsc::channel();

  let th = thread::spawn(move || {
    let mut count = 0;
    for _i in 1..11 {
      count = count + 1;
      thread::sleep(Duration::from_millis(100));
    }
    //send value tx->rx
    tx.send(count).unwrap();
  });

  let mut cnt = 0;
  for _i in 1..11 {
    cnt = cnt + 1;
    thread::sleep(Duration::from_millis(100));
  }

  //wait for thread
  th.join().unwrap();

  //recieve value from rx
  let val = rx.recv().unwrap();

  println!("counter:{}", cnt + val);
  println!("finish!");
}