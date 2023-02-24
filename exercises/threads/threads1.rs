// threads1.rs
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a hint.

// This program spawns multiple threads that each run for at least 250ms,
// and each thread returns how much time they took to complete.
// The program should wait until all the spawned threads have finished and
// should collect their return values into a vector.


use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut handles = vec![];
    for i in 0..10 {
        handles.push(thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i);
            start.elapsed().as_millis()
        }));
    }
    // 这时 handles 内容如下：
    // [JoinHandle { .. }, JoinHandle { .. }, JoinHandle { .. }, JoinHandle { .. },
    // JoinHandle { .. }, JoinHandle { .. }, JoinHandle { .. }, JoinHandle { .. },
    // JoinHandle { .. }, JoinHandle { .. }]

    let mut results: Vec<u128> = vec![];
    for handle in handles {
        // TODO: a struct is returned from thread::spawn, can you use it?
        // handle执行完成后，返回值被push到results
        results.push(handle.join().unwrap());
    }

    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }
    
    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} took {}ms", i, result);
    }
}
