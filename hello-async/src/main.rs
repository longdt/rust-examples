use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("Hello before reading file!");
    let handle1 = thread::spawn(|| {
        let file_content = read_file();
        println!("{:?}", file_content);
    });
    let handle2 = thread::spawn(|| {
        let other_file_content = read_other_file();
        println!("{:?}", other_file_content);
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
}

fn read_other_file() -> String {
    sleep(Duration::new(4, 0));
    String::from("Hello, there from other file")
}

fn read_file() -> String {
    sleep(Duration::new(2, 0));
    String::from("Hello, there from file")
}
