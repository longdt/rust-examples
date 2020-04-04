#[allow(unused_variables)]

fn main() {
    //all have type `Option<i32>`
    let number = Some(7);
    let letter: Option<i32> = None;
    let emotion: Option<i32> = None;

    //the `if let` construct reads: "if `let` destructures `number` into `Some(i)`,
    //evaluate block (`{}`).
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    //if you need to specify a failure, use an else:
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        //destructure failed. Change to the failure case.
        println!("Didn't match a number. Let's go with a letter!");
    }

    //provide an altered failing condition
    let i_like_letters = false;

    if let Some(i) = emotion {
        println!("Matched {:?}!", i);
    //destructure failed. Evaluate an `else if` condition to see if the alternate failure branch
    //should be taken:
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        //the condition evaluated false. This branch is default:
        println!("I don't like letters. Let's go with an emoticon :)!");
    }

    //create example variables
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    //variable a matches Foo::Bar
    if let Foo::Bar = a {
        println!("a is foobar");
    }

    //variable b doesn't match Foo::Bar so this will print nothing
    if let Foo::Bar = b {
        println!("b is foobar");
    }

    //variable c matches Foo::Qux which has a value similar to Some() in previous example
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    //binding also works with `if let`
    if let Foo::Qux(value @ 100) = c {
        println!("c is one hundred: {}", value);
    }

    let aev = AnEnum::AVariant;

    //variable aev matches AnEnum::AVariant
    // if AnEnum::AVariant == aev {
    //     //^-- this causes a compile-time error. Use `if let` instead.
    //     println!("aev is avariant");
    // }
}

enum Foo {
    Bar,
    Baz,
    Qux(u32),
}

// This enum purposely doesn't #[derive(PartialEq)],
// neither we implement PartialEq for it. That's why comparing AnEnum::AVariant == aev fails.
enum AnEnum {
    AVariant,
}
