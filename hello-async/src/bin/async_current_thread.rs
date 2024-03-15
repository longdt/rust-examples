use std::thread;
use std::time::Duration;

#[tokio::main(flavor="current_thread")]
async fn main() {
    println!("{:?}: Hello before reading file!", thread::current().id());

    let h1 = tokio::spawn(async {
        let _file_content = read_file().await;
    });

    let h2 = tokio::spawn(async {
        let _other_file_content = read_other_file().await;
    });
    let _ = tokio::join!(h1, h2);
}

async fn read_file() -> String {
    tokio::time::sleep(Duration::new(10, 0)).await;
    println!("{:?}: {:?}", thread::current().id(), "Processing file");
    String::from("Hello, there from file")
}

async fn read_other_file() -> String {
    tokio::time::sleep(Duration::new(2, 0)).await;
    println!("{:?}: {:?}", thread::current().id(), "Processing other file");
    String::from("Hello, there from other file")
}
