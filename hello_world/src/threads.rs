use std::thread;

fn main() {
  let guards: Vec<_> = (0..10).map(|_| {
    thread::scoped(|| {
      println!("Hello, world!");
    })
  }).collect();
}
