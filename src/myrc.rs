use std::rc::Rc;
// reference counter, every variable can only have one owner
fn takes_a_string(input: String) {
    println!("It is: {}", input);
}
fn also_takes_a_string(input: String) {
    println!("It is: {}", input);
}

#[derive(Debug)]
struct City {
    name: String,
    population: u32,
    history: Rc<String>,
}
#[derive(Debug)]
struct CityData {
    names: Vec<String>,
    histories: Vec<Rc<String>>,
}
fn main() {
    let username = String::from("User MacUser");
    takes_a_string(username);
    // Rc lets you have more than one owner.
    // Once the number of owners goes down to 0, the variable disappear.
    //also_takes_a_string(username);

    let calgary = City {
        name: "Calgary".to_string(),
        population: 1_200_000, 
        history: Rc::new("Calgary began as a fort called Fort Calgary that...".to_string()), // long
    };
    let canada_cities = CityData {
        names: vec![calgary.name],
        histories: vec![calgary.history.clone()], // .clone() to add a new reference 
    };
    println!("{}", calgary.history);;
    println!("{}", Rc::strong_count(&calgary.history));
    let new_owner = calgary.history.clone();
}
