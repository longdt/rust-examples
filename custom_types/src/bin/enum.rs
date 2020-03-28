fn main() {
    let pressed = WebEvent::KeyPress('x');
    //`to_owned` creates an owned `String` from a str slice.
    let pasted = WebEvent::Paste("my test".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}

//create an `enum` to classify a web event. Note how both names and types information together specify
//the variant: `PageLoad` != `PageUnload` and `KeyPress(char) != Paste(String)`.
#[derive(Debug)]
enum WebEvent {
    //An `enum variant` may either be `unit-like`,
    PageLoad,
    PageUnload,

    //like tuple structs
    KeyPress(char),
    Paste(String),

    //or c-like structures
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    //error[E0369]: binary operation `==` cannot be applied to type `WebEvent`
    // let is_page_load = event == WebEvent::PageLoad;
    println!("event {:?}", event);

    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        WebEvent::Click { x, y } => println!("clicked at x = {}, y = {}", x, y),
    }
}
