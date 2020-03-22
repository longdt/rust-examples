use crate::WebEvent::PageLoad;

fn main() {}

//create an `enum` to classify a web event. Note how both names and types information together specify
//the variant: `PageLoad` != `PageUnload` and `KeyPress(char) != Paste(String)`.
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
    let is_page_load = (event == PageLoad);
    println!("page loaded {}", is_page_load);
}
