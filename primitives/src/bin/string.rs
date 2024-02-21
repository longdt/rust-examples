fn main() {
    let s = String::new();
    let s1: String = "".into();
    if s == s1 {
        println!("equal");
    } else {
        println!("not equal");
    }
}
