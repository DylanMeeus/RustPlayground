mod machine;

fn main() {
    let mut machine = machine::enigma::create_machine();
    machine.encrypt("hello".to_string());
    match machine.get_rotor("IC".to_string()) {
        Some(r) => {
            println!("{}", r.position);
            r.incr_pos();
            println!("{}", r.position);

        },
        None => println!("oopsie")
    };
}
