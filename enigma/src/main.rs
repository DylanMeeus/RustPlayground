mod machine;

fn main() {
    let mut machine = machine::enigma::create_machine();
    machine.encrypt("hello".to_string());
    match machine.get_rotor("IC".to_string()) {
        Some(r) => {
            println!("{}", r.position);
            r.set_pos(12);
            println!("{}", r.position);

        },
        None => println!("oopsie")
    };
}
