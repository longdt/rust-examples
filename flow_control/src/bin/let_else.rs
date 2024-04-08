use std::str::FromStr;

fn get_count_item(s: &str) -> (u64, &str) {
    let mut it = s.split(' ');
    let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
        println!("Can't segment count item pair: '{s}'");
        panic!("Can't segment count item pair: '{s}'");
        // TODO ^ Try comment this line
        // Error! expected `!`, found `()`
    };
    let Ok(count) = u64::from_str(count_str) else {
        panic!("Can't parse integer: '{count_str}'");
    };
    (count, item)
}

fn main() {
    assert_eq!(get_count_item("3 chairs"), (3, "chairs"));
    assert_eq!(get_count_item("3chairs"), (3, "chairs"));
}
