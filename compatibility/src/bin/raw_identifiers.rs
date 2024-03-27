// Error! `try` is a reserved keyword in Rust 2018 edition
// fn try() {
// }

fn r#try() {
    println!("Hello try");
}
fn main() {
    // Error! expected identifier, found keyword `try`
    // try();
    // TODO ^ Try uncommenting this line
    r#try();
}
