mod machine;

fn main() {
    let mut encrypt_machine = machine::enigma::create_machine();
    let encrypted = encrypt_machine.encrypt("HELLO world".to_string());

    println!("{:?}", encrypted);

    let mut decrypt_machine = machine::enigma::create_machine();
    let decrypted = decrypt_machine.encrypt(encrypted);
    println!("{:?}", decrypted);
}
