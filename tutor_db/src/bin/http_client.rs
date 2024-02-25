use ntex::http::Client;
use std::io;

#[ntex::main]
async fn main() -> io::Result<()> {
    let client = Client::default();
    let mut resp = client.get("https://google.com.vn").send().await.unwrap();
    let body = resp.body().await.unwrap();
    println!("{:?}", body);
    Ok(())
}
