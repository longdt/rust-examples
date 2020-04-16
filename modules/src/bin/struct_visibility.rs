fn main() {
    //public structs with public fields can be constructed as usual
    let open_box = my::OpenBox {
        contents: "public information",
    };

    //and their fields can be normally accessed.
    println!("The open box contains: {}", open_box.contents);

    //public structs with private fields cannot be constructed using field names.
    //error! `ClosedBox` has private fields
    // let closed_box = my::ClosedBox {
    //     contents: "classified information",
    // };
    //TODO ^ try uncommenting this line

    //however, structs with private fields can be created using public constructors
    let _closed_box = my::ClosedBox::new("classified information");

    //and the private fields of a public struct cannot be accessed.
    //error! the `contents` field is private
    // println!("The closed box contains: {}", _closed_box.contents);
    //TODO ^ try uncommenting this line
}

mod my {
    //a public struct with  public field of generic type `T`
    pub struct OpenBox<T> {
        pub contents: T,
    }

    //a public struct with a private field of generic type `T`
    #[allow(dead_code)]
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        //a public constructor method
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox { contents }
        }
    }
}
