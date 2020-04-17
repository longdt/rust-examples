//link to `libcrates`, import items under the `crates` module
extern crate crates as rary;

fn main() {
    rary::public_function();

    //error! `private_function` is private
    // rary::private_function();

    rary::indirect_access();
}