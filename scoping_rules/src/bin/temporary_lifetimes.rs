#[derive(Debug)]
struct MyString(String);

trait ToMyString {
    fn to_my_string(&self) -> MyString;
}

impl ToMyString for usize {
    fn to_my_string(&self) -> MyString {
        MyString(format!("ToMyString: {}", *self))
    }
}

impl From<&str> for MyString {
    fn from(value: &str) -> Self {
        Self(String::from(value))
    }
}

impl From<char> for MyString {
    fn from(value: char) -> Self {
        Self(String::from(value))
    }
}

impl From<usize> for MyString {
    fn from(value: usize) -> Self {
        Self(value.to_string())
    }
}

impl Drop for MyString {
    fn drop(&mut self) {
        println!("MyString: {} was dropped", self.0)
    }
}
fn main() {
    let s = MyString::from("main function lifetime. Print before main return");
    //Temporaries
    //temporary value is freed at the end of this statement
    println!("----- case A ----");
    MyString::from("temporary A");
    println!("----- case B ----");
    let _ = MyString::from("temporary B");
    println!("----- case C ----");
    &MyString::from("temporary C");

    //temporary A will be dropped after method fn_return_void
    //the MyString is first allocated, then a reference to it is passed to fn_return_void,
    // and only after fn_return_void returns will we drop the temporary fn_return_void.
    println!("----- case D ----");
    fn_return_void(&MyString::from("temporary D"));


    //In a let statement
    println!("----- case E ----"); //temporary value is freed at the end of this statement
    let e = fn_return_usize(&MyString::from("temporary E"));
    println!("case E: usize = {}", e);

    //Due variable c wasn't used. So compiler ignore return value and temporary value is freed at the end of this statement
    println!("----- case F ----");
    let f = fn_return_ref(&MyString::from("temporary F"));
    //error[E0716]: temporary value dropped while borrowed
    //temporary value G is freed at the end of this statement. But borrow later used here
    println!("----- case G ----");
    // let g = fn_return_ref(&MyString::from("temporary G"));
    // println!("case G: array lenth = {} and first char = {}", g.len(), g[0]);

    //same as case E: temporary value is freed at the end of this statement
    println!("----- case H ----");
    let h = fn_return_static_ref(&MyString::from("temporary H"));
    println!("case H: {}", h);

    //In a nested call
    //g(f(&String::from('🦀')));
    //regardless of the signature of f, the String is kept alive until the end of the statement, until after g is called.
    println!("----- case I ----");
    MyString::from(fn_return_usize(&MyString::from("temporary I")));
    println!("----- case J ----");
    fn_return_usize(&MyString::from("temporary J")).to_my_string();
    println!("Hello world!");
}

fn fn_return_void(s: &MyString) {
    println!("{:?} enter function: fn_return_void", s);
}

fn fn_return_usize(s: &MyString) -> usize {
    println!("{:?} enter function: fn_return_usize", s);
    s.0.len()
}

fn fn_return_ref(s: &MyString) -> &[u8] {
    println!("{:?} enter function: fn_return_ref", s);
    s.0.as_bytes()
}

fn fn_return_static_ref(s: &MyString) -> &'static str {
    println!("{:?} enter function: fn_return_static_ref", s);
    "static ref: hehe"
}
