mod machine;

fn main() {
    let mut encrypt_machine = machine::enigma::create_machine();
    let encrypted = encrypt_machine.encrypt("HELLO".to_string());

    println!("{:?}", encrypted);

    let mut decrypt_machine = machine::enigma::create_machine();
    let decrypted = decrypt_machine.encrypt(encrypted);
    println!("{:?}", decrypted);
    /*
    match machine.get_rotor("IC".to_string()) {
        Some(r) => {
            println!("{}", r.position);
            r.incr_pos();
            println!("{}", r.position);
            println!("{}", r.get_input_at_pin(8 as u8));

        },
        None => println!("oopsie")
    };
    */
}
