trait Person {
    fn name(&self) -> String;
}

// Person is a supertrait of Student.
// Implementing Student requires you to also impl Person.
trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

// CompSciStudent (computer science student) is a subtrait of both Programmer and Student.
// Implementing CompSciStudent requires you to impl both supertraits.
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

struct MitStudent;

impl Programmer for MitStudent {
    fn fav_language(&self) -> String {
        String::from("Rust")
    }
}

impl Student for MitStudent {
    fn university(&self) -> String {
        String::from("MIT")
    }
}

impl Person for MitStudent {
    fn name(&self) -> String {
        String::from("Long")
    }
}

impl CompSciStudent for MitStudent {
    fn git_username(&self) -> String {
        String::from("longdt")
    }
}

fn main() {
    let student = MitStudent;
    println!("> {}", comp_sci_student_greeting(&student));
}
