//atomic
use std::thread;
use std::time::Duration;
use std::sync::atomic::{self, AtomicU32, Ordering};

static COUNTER: AtomicU32 = AtomicU32::new(0);

fn count_up() {
  COUNTER.fetch_add(1, atomic::Ordering::SeqCst);
}

fn get_count() -> u32 {
  return COUNTER.load(Ordering::SeqCst);
}

fn main() {
  let th1 = thread::spawn(move || {
    for _i in 1..11 {
      print!("+");
      count_up();
      thread::sleep(Duration::from_millis(100));
    }
  });

  let th2 = thread::spawn(move || {
    for _i in 1..11 {
      print!("-");
      count_up();
      thread::sleep(Duration::from_millis(100));
    }
  });

  th1.join().unwrap();
  th2.join().unwrap();
  println!("\ncounter={}", get_count());
}