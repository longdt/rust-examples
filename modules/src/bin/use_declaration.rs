#[allow(unused_imports)]
//bring functions/traits to scope
use crate::deeply::nested::{my_first_function, my_second_function, AndATraitType};

//bind the `deeply::nested::function` path to `other_function`
use deeply::nested::function as other_function;

fn main() {
    my_first_function();

    //easier access to `deeply_nested::function`
    other_function();

    println!("Entering block");
    {
        //this is equivalent to `use deeply::nested::function as function`.
        //this `function()` will shadow the outer one.
        use crate::deeply::nested::function;
        function();

        //`use` bindings have a local scope. In this case, the shadowing of `function()` is only in
        //this block.
        println!("Leaving block");
    }
    function();
}

fn function() {
    println!("called `function()`");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`");
        }

        pub fn my_first_function() {
            println!("called `deeply::nested::my_first_function()`");
        }

        #[allow(dead_code)]
        pub fn my_second_function() {}

        pub trait AndATraitType {}
    }
}
