fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
        tall: i32,
    }
    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
        tall: 170,
    };

    //`name` is moved out of person, but `age` is referenced
    let Person {
        name,
        ref age,
        tall,
    } = person;
    println!("The person's age is {}", age);
    println!("The person's name is {}", name);
    println!("The person's tall is {}", tall);

    // Error! borrow of partially moved value: `person` partial move occurs
    // println!("The person struct is {:?}", person);
    println!("The person's age from person struct is {}", person.age);
    println!("The person's tall from person struct is {}", person.tall);

    #[derive(Debug)]
    struct Student {
        id: u32,
        class: String,
    }

    let student = Student {
        id: 1000,
        class: String::from("Math"),
    };

    //`name` is moved out of person, but `age` is referenced
    let Student { id, class } = student;
    println!("The student's id is {}", id);
    println!("The student's class is {}", class);

    // Error! borrow of partially moved value: `student` partial move occurs
    println!("The student struct is {:?}", student);
    println!("The student's id from student struct is {}", student.id);
}
