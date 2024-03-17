use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

#[derive(Copy, Clone)]
struct SuccessFuture;

impl Future for SuccessFuture {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("{:?}: Tokio! Stop polling me", thread::current().id());
        cx.waker().wake_by_ref();
        Poll::Ready(String::from("Hello, there from success future"))
    }
}

#[tokio::main]
async fn main() {
    println!("{:?}: Hello before reading file!", thread::current().id());

    let h1 = tokio::spawn(async {
        let future = SuccessFuture;
        println!("{:?}: Inside async block before future.await", thread::current().id());
        let file_content = future.await;
        // Since future.await will call fn IntoFuture::into_future(self) -> Self::IntoFuture
        // so Future need be Copy to call .await multiple time
        let other_file_content = future.await;
        other_file_content
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
