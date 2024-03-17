use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

struct RepeatPollFuture;

impl Future for RepeatPollFuture {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("{:?}: Tokio! Stop polling me", thread::current().id());
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

#[tokio::main]
async fn main() {
    println!("{:?}: Hello before reading file!", thread::current().id());

    let h1 = tokio::spawn(async {
        let future = RepeatPollFuture;
        println!("{:?}: Inside async block before future.await", thread::current().id());
        future.await
    });

    let h2 = tokio::spawn(async {
        let file_content = read_file().await;
        println!("{:?}: {:?}", thread::current().id(), file_content);
    });

    println!("{:?}: Before join", thread::current().id());
    let _ = tokio::join!(h1, h2);
}

async fn read_file() -> String {
    sleep(Duration::new(2, 0));
    println!("{:?}: {:?}", thread::current().id(), "Processing file");
    String::from("Hello, there from file")
}
