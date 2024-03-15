use std::future::Future;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("{:?}: Hello before reading file!", thread::current().id());

    let h1 = tokio::spawn(async {
        let _file_content = read_file().await;
    });

    let h2 = tokio::spawn(async {
        let _other_file_content = read_other_file().await;
    });
    // let _ = tokio::join!(h1, h2);
    sleep(Duration::new(10, 0));
}

async fn read_file() -> String {
    tokio::time::sleep(Duration::new(4, 0)).await;
    println!("{:?}: {:?}", thread::current().id(), "Processing file");
    String::from("Hello, there from file")
}

fn read_other_file() -> impl Future<Output = String> {
    async {
        sleep(Duration::new(2, 0));
        println!(
            "{:?}: {:?}",
            thread::current().id(),
            "Processing other file"
        );
        String::from("Hello, there from other file")
    }
}
