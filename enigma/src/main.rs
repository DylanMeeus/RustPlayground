mod machine;

fn main() {
    let mut machine = machine::enigma::create_machine();
    machine.encrypt("hello".to_string());
    match machine.get_rotor("IC".to_string()) {
        Some(r) => println!("{}", r.id),
        None => println!("oopsie")
    };
}
