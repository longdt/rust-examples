fn main() {
    //Error: `UnPrintable` doesn't implement `std::fmt::Debug`
    // println!("{:?}", UnPrintable(1));
    println!("{0:?} and pretty print: {0:#?}", DebugPrintable(2));


}

//this structure can't be printed with fmt::Display or fmt::Debug
#[allow(dead_code)]
struct UnPrintable(i32);
//`derive` attribute automatically implement fmt::Debug make this struct printable
#[derive(Debug)]
struct DebugPrintable(i32);

//derive(Debug) require all field must implement fmt::Debug
// #[derive(Debug)]
// struct Deep {
//     up: UnPrintable
// }