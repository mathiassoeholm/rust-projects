use std::sync::Arc;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use web_server::ThreadPool;

#[test]
fn it_runs_all_jobs() {
  let num_threads = 3;
  let thread_pool = ThreadPool::new(2);
  let threads_executed = Arc::new(Mutex::new(0));

  for _ in 0..num_threads {
    let threads_executed = Arc::clone(&threads_executed);
    thread_pool.execute(move || {
      let mut threads_executed = threads_executed.lock().unwrap();
      *threads_executed += 1;
    });
  }

  let start = Instant::now();
  while start.elapsed() < Duration::from_secs(1) {
    if *threads_executed.lock().unwrap() == num_threads {
      return;
    };
  }

  panic!(
    "Expected {} threads to run, but only {} ran",
    num_threads,
    *threads_executed.lock().unwrap()
  );
}
