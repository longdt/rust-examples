struct Droppable;

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Droppable was dropped");
    }
}

fn drink(beverage: &str) {
    // You shouldn't drink too much sugary beverages.
    if beverage == "lemonade" {
        panic!("AAAaaaaa!!!!!");
    }

    println!("Some refreshing {} is all I need.", beverage);
}

fn main() {
    let _d = Droppable;
    drink("water");
    drink("lemonade");
    drink("still water");
}
