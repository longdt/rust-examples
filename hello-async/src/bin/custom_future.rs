use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

struct ReadFileFuture;

impl Future for ReadFileFuture {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("{:?}: Tokio! Stop polling me", thread::current().id());
        Poll::Pending
    }
}

#[tokio::main]
async fn main() {
    println!("{:?}: Hello before reading file!", thread::current().id());

    let h1 = tokio::spawn(async {
        let future1 = ReadFileFuture;
        println!("{:?}: Inside async block before future1.await", thread::current().id());
        future1.await
    });

    let h2 = tokio::spawn(async {
        let file_content = read_file().await;
        println!("{:?}: {:?}", thread::current().id(), file_content);
    });

    let h3 = tokio::spawn(ReadFileFuture);
    println!("{:?}: Before join", thread::current().id());
    let _ = tokio::join!(h1, h2, h3);
}

fn read_file() -> impl Future<Output = String> {
    async {
        sleep(Duration::new(2, 0));
        println!("{:?}: {:?}", thread::current().id(), "Processing file");
        String::from("Hello, there from file")
    }
}
