fn main() {
    println!("Address of constant: &1 = {:p}", &1);
    println!("Address of constant: &2 = {:p}", &2);
    println!("Address of constant: &'hello' = {:p}", &"hello");
    let a = 1;
    let b = 2;
    let c =  "hello";
    println!("Address of constant: &a = {:p}", &a);
    println!("Address of constant: &b = {:p}", &b);
    println!("Address of constant: &c = {:p}", &c);
    let d = Box::new(10);
    println!("Address of constant: &d = {:p}", d);
}
