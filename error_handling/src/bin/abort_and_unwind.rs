struct Droppable;

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Droppable was dropped");
    }
}

fn drink(beverage: &str) {
    // You shouldn't drink too much sugary beverages.
    if beverage == "lemonade" {
        if cfg!(panic = "abort") {
            println!("This is not your party. Run!!!!");
        } else {
            println!("Spit it out!!!!");
        }
    } else {
        println!("Some refreshing {} is all I need.", beverage);
    }
}

#[cfg(panic = "unwind")]
fn ah() {
    println!("Spit it out!");
}

#[cfg(not(panic = "unwind"))]
fn ah() {
    println!("This is not your party. Run!");
}

fn drink1(beverage: &str) {
    if beverage == "lemonade" {
        ah();
    } else {
        println!("Some refreshing {} is all I need.", beverage);
    }
}
fn main() {
    let _d = Droppable;
    drink("water");
    drink("lemonade");

    drink1("water");
    drink1("lemonade");
    panic!("Aaaa");
}
