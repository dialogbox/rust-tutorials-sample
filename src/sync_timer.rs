use std::{thread, time};

pub fn sleep<F, R>(duration: time::Duration, f: F) -> R
where
  F: FnOnce() -> R,
{
  thread::sleep(duration);
  f()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_sleep() {
    let dur = time::Duration::from_millis(300);
    let now = time::Instant::now();

    let result = sleep(dur, || "Hello Sync Timer");

    assert!(now.elapsed() >= dur);
    assert_eq!(result, "Hello Sync Timer");
  }
}
