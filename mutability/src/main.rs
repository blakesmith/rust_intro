struct Person {
    name: String,
    salutation: String,
}

fn rename(person: &mut Person, new_name: String) {
    person.name = new_name;
}

fn say_hello(person: &Person) {
    println!("{} {}", person.salutation, person.name);
}

// fn illegal_references() {
//     let mut person = Person {
//         name: "Mary".to_string(),
//         salutation: "Bonjour".to_string()
//     };
//     let mut p1 = &mut person;
//     let mut p2 = &mut person;
// }

// fn reassign_immutable() {
//     let name = "Josephine";
//     name = "Billy";
// }

fn main() {
    let mut person = Person {
        name: "Gary".to_string(),
        salutation: "Hello".to_string()
    };
    say_hello(&person);
    rename(&mut person, "Larry".to_string());
    say_hello(&person);
}


