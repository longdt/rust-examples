//an attribute to hide warnings for unused code.
#![allow(dead_code)]

fn main() {
    //explicitly `use` each name so they are available without manual scoping.
    use Status::{Poor, Rich};
    //automatically `use` each name inside `Work`.
    use crate::Work::*;

    //equivalent to `Status::Poor`
    let status = Poor;
    //equivalent to `Work::Civilian`
    let work = Civilian;

    match status {
        //note the lack of scoping because of explicit `use` above.
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        //note again the lack of scoping.
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldier fight!"),
    }
}

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}
