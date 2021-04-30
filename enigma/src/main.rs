use std::io::{stdin, stdout, Write};
mod machine;

fn main() {
    run();
}


fn run() {
    // create both machines with default config 
    let mut encrypt_machine = machine::enigma::create_machine();
    let mut decrypt_machine = machine::enigma::create_machine();


    let mut s = String::new();
    print!("Enter text to encrypt: ");
    let _ = stdout().flush();

    stdin().read_line(&mut s).expect("An input string was expected");

    // handle carriage return
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }

    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }


    let encrypted = encrypt_machine.encrypt(s);
    println!("{:?}", encrypted);
    let decrypted = decrypt_machine.encrypt(encrypted);
    println!("{:?}", decrypted);
}
