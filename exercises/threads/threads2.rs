// threads2.rs
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a hint.
// Building on the last exercise, we want all of the threads to complete their work but this time
// the spawned threads need to be in charge of updating a shared value: JobStatus.jobs_completed

// status_shared.lock().unwrap()
// status_shared 的类型是 Arc<Mutex<JobStatus>
// .lock() 后的类型 怎么变成了 LockResult<MutexGuard<JobStatus>> ?
// Arc 呢 ？
// 答案在 let status_shared = Arc::clone(&status);
// 在 Arc 上调用 clone 会生成一个新的 Arc 实例，
// 该实例指向堆上与源 Arc 相同的分配，同时增加了引用计数。
// lock() 是作用在 Mutex<JobStatus> 上，返回了 LockResult<MutexGuard<JobStatus>>

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            let mut counter = status_shared
                .lock()
                .unwrap();
            counter.jobs_completed += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        // interesting in the output? Do you have to 'join' on all the handles?
        let completed = status
            .lock()
            .unwrap()
            .jobs_completed;

        println!("jobs completed {}", completed);
    }
}
