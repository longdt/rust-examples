use derive_from::From;

struct Person {
    name: &'static str,
    age: u8,
}

#[derive(From)]
#[from(Person)]
struct PersonResponse {
    name: &'static str,
    age: u8,
}

#[test]
fn test_from() {
    let person = Person {
        name: "long",
        age: 30,
    };
    let response = PersonResponse::from(person);
    assert_eq!(response.name, "long");
    assert_eq!(response.age, 30);
}