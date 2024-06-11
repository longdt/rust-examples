use lists::first::List;

fn main() {
    let mut list = List::new();
    // assert_eq!(list.pop(), None);
    fn drop_list(list: List) {
        println!("--------------");
    }
    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);
    list.push(5);
    list.push(6);
    list.push(7);
    list.push(8);
    list.push(9);
    list.push(10);
    list.push(11);
    list.push(12);
    list.push(13);
    list.push(14);
    list.push(15);
    list.push(16);
    list.push(17);
    list.push(18);
    list.push(19);
    drop_list(list);
    println!("-----end-----");
}
