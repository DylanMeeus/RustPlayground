use std::fs;

#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[allow(non_snake_case)]
type pins = (u8, u8);

use super::rotors;
use super::tools;

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
        let ci = tools::char_to_idx(i.0.chars().next().unwrap());
        let ci2 = tools::char_to_idx(i.1.chars().next().unwrap());
        pins.push((ci, ci2));
    }

    plugboard{plugs: pins}
}

pub struct enigma_machine {
    pub plugboard: plugboard,
    pub rotors: Vec<rotors::rotor>
}


impl enigma_machine {

    pub fn encrypt(&mut self, input: String) {
        let chars: Vec<char> = input.to_uppercase().chars().collect();

        // send each char through the rotors
        for c in chars {
            let mut result = c;
            for rotor in &self.rotors {
                result = rotor.map(result);
            }
            println!("mapped {} to char: {}", c, result);
        }
    }

    pub fn get_rotor(&mut self, ID: String) -> Option<&mut rotors::rotor> {
        for (i, r) in self.rotors.iter().enumerate() {
            if r.id == ID {
                return Some(&mut self.rotors[i])
            }
        }
        None
    }
}


/// create_machine constructs an enigma machine with rotor configurations from:
/// https://en.wikipedia.org/wiki/Enigma_rotor_details
pub fn create_machine() -> enigma_machine {
    let IC = vec!['D','M','T','W','S','I','L','R','U','Y','Q','N','K','F','E','J','C','A','Z','B','P','G','X','O','H','V'];
    let first_rotor = rotors::create_rotor("IC".to_string(), IC);

    let IIC = vec!['H','Q','Z','G','P','J','T','M','O','B','L','N','C','I','F','D','Y','A','W','V','E','U','S','R','K','X'];
    let second_rotor = rotors::create_rotor("IIC".to_string(), IIC);

    let rs = vec![first_rotor, second_rotor];
    let pboard = create_plugboard("src/plug.board".to_string());
    enigma_machine{plugboard: pboard, rotors: rs}
}
