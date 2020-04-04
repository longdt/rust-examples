fn main() {
    //for in with range
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    //for in with iterators
    let mut vec = vec!["Bob", "Frank", "Ferris"];
    for &name in vec.iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    for name in vec.iter_mut() {
        *name = match *name {
            "Ferris" => "There is a rustacean among us!",
            "Frank" => "Frank",
            _ => "Hello",
        }
    }
    println!("names: {:?}", vec);

    for name in vec.into_iter() {
        match name {
            "Frank" => println!("Frank is a rustacean"),
            _ => println!("Hello {}", name),
        }
    }
}
