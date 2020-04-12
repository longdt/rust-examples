#[allow(
    unused_variables,
    unreachable_code,
    clippy::let_unit_value,
    clippy::diverging_sub_expression
)]
fn main() {
    let a = some_fn();
    println!("This function returns and you can see this line.");

    println!(
        "Sum of odd numbers up to 9 (excluding): {}",
        sum_odd_numbers(9)
    );

    let x = foo();
    println!("You will never see this line!");
}

#[allow(clippy::unused_unit)]
fn some_fn() {
    ()
}

fn foo() -> ! {
    panic!("This call never returns");
}

#[allow(clippy::match_bool)]
fn sum_odd_numbers(up_to: u32) -> u32 {
    let mut acc = 0;
    for i in 0..up_to {
        //notice that the return type of this match expression must be u32
        //because of the type of the `addition` variable.
        let addition: u32 = match i % 2 == 1 {
            //the `i` variable is of type u32, which is perfectly fine.
            true => i,
            //on the other hand, the `continue` expression doesn't return u32,
            //but it is still fine, because it never returns and therefore doesn't not
            //violate the type requirements of the match expression
            false => continue,
        };
        acc += addition;
    }
    acc
}
