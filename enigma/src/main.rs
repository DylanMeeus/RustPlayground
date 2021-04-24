use enigma::{create_plugboard, rotor};

#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[allow(non_snake_case)]

mod enigma {
    use std::fs;

    type rotor_pins = (u8, u8);
    type pins = (u8, u8);

    pub struct rotor {
        // rotor configuration
        pub id: String,
        pub wiring: Vec<rotor_pins> ,
        pub position: u8
    }

    pub struct plugboard {
        // plugboard configuration
        plugs: Vec<pins>
    }

    /// create_plugboard reads the plugboard configuration in the provided file
    pub fn create_plugboard(filename: String) -> plugboard {
        let input = fs::read_to_string(filename).expect("could not read file");
        let lines: Vec<String> = input.split("\n").map(str::to_string).collect();
        let base = lines[0].split(" ");
        let map = lines[1].split(" ");

        let mut pins: Vec<pins> = vec!();
        for i in base.zip(map) {
            let ci = char_to_idx(i.0.chars().next().unwrap());
            let ci2 = char_to_idx(i.1.chars().next().unwrap());
            pins.push((ci, ci2));
        }

        plugboard{plugs: pins}
    }

    pub struct enigma_machine {
        pub plugboard: plugboard,
        pub rotors: Vec<rotor>
    }


    impl enigma_machine {
        pub fn encrypt(&self, input: String) {
            println!("{:?}", input);
        }

        pub fn get_rotor(&mut self, ID: String) -> Option<&mut rotor> {
            for (i, r) in self.rotors.iter().enumerate() {
                if r.id == ID {
                    return Some(&mut self.rotors[i])
                }
            }
            None
        }
    }

    pub fn char_to_idx(c: char) -> u8 {
        // todo: add checks to verify c is in range [aA-zZ]
        let ci = c as u8;
        // keep in mind lower-case & uppercase characters
        if ci >= 97 {
            return ci - 97;
        }
        ci - 65
    }

    /// create_rotor_wiring assumes wiring each element in "input" against the alphabet from
    /// A-Z
    fn create_rotor_wiring(input: Vec<char>) -> Vec<rotor_pins> {
        let mut idx = 0;
        input.into_iter()
            .map(|x| { idx += 1; (idx, x as u8) }).
            collect()
    }

    fn create_rotor(name: String, input: Vec<char>) -> rotor {
        let rotor_wiring = create_rotor_wiring(input);
        rotor {
            id: name,
            wiring: rotor_wiring,
            position: 0
        }
    }

    /// create_machine constructs an enigma machine with rotor configurations from:
    /// https://en.wikipedia.org/wiki/Enigma_rotor_details
    pub fn create_machine() -> enigma_machine {
        let IC = vec!['D','M','T','W','S','I','L','R','U','Y','Q','N','K','F','E','J','C','A','Z','B','P','G','X','O','H','V'];
        let first_rotor = create_rotor("IC".to_string(), IC);

        let IIC = vec!['H','Q','Z','G','P','J','T','M','O','B','L','N','C','I','F','D','Y','A','W','V','E','U','S','R','K','X'];
        let second_rotor = create_rotor("IIC".to_string(), IIC);

        let rs = vec![first_rotor, second_rotor];
        let pboard = create_plugboard("src/plug.board".to_string());
        enigma_machine{plugboard: pboard, rotors: rs}
    }

}

fn main() {
    let mut machine = enigma::create_machine();
    machine.encrypt("hello".to_string());
    match machine.get_rotor("IC".to_string()) {
        Some(r) => println!("{}", r.id),
        None => println!("oopsie")
    };
}
