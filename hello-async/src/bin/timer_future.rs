use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant};

struct AsyncTimer {
    expiration_time: Instant,
}

impl Future for AsyncTimer {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.expiration_time {
            println!("{:?}: Hello, it's time for Future 1", thread::current().id());
            Poll::Ready(String::from("Future 1 has completed"))
        } else {
            println!("{:?} Hello, it's not yet time for Future 1. Going to sleep", thread::current().id());
            let waker = cx.waker().clone();
            let expiration_time = self.expiration_time;
            thread::spawn(move || {
                let current_time = Instant::now();
                if current_time < expiration_time {
                    thread::sleep(expiration_time - current_time);
                }
                println!("{:?}: wake timer", thread::current().id());
                waker.wake();
            });
            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    println!("{:?}: Hello before reading file!", thread::current().id());
    let h1 = tokio::spawn(async {
        let future1 = AsyncTimer {
            expiration_time: Instant::now() + Duration::from_secs(4),
        };
        println!("{:?}: {:?}", thread::current().id(), future1.await);
    });

    let h2 = tokio::spawn(async {
        let file_content = read_file().await;
        println!("{:?}: {:?}", thread::current().id(), file_content);
    });
    println!("{:?}: Before join", thread::current().id());
    let _ = tokio::join!(h1, h2);
}

fn read_file() -> impl Future<Output = String> {
    async {
        sleep(Duration::new(2, 0));
        String::from("Future 2 has completed")
    }
}
