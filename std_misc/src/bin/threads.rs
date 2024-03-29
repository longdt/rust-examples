use std::thread;

const NTHREADS: u32 = 10;

// This is the `main` thread
fn main() {
    // Make a vector to hold the children which are spawned.
    let mut children = vec![];
    println!("{:?} This main which has id: {:?}", thread::current().name(), thread::current().id());
    for i in 0..NTHREADS {
        // Spin up another thread
        children.push(thread::spawn(move || {
            println!("{:?}: This is thread id: {:?} which has number {}", thread::current().name(), thread::current().id(), i);
        }));
    }
    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
}
