struct Person {
    name: String,
    salutation: Option<String>,
}

fn hello(person: &Person) {
    match person.salutation {
        Some(ref salutation) => println!("{}, {}!",
                                         salutation, person.name),
        None => println!("Welcome {}!", person.name)
    }
}

fn main() {
    let person = Person {
        name: "Jill".to_string(),
        salutation: Some("Hola".to_string())
    };
    hello(&person);
}
