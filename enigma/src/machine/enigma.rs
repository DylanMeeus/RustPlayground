use std::fs;
use std::collections::HashMap;

#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[allow(non_snake_case)]
type pins = (char, char);

use super::rotors;
use super::tools;

pub struct plugboard {
    // plugboard configuration
    pub plugs: Vec<pins>,
    pub output_to_static_rotor: HashMap<char, u8>
}

pub struct reflector {
    pub mapping: HashMap<char, char>
}

/// create_reflector constructs a bidirectional mapping
pub fn create_reflector() -> reflector {

    let mut m: HashMap<char, char> = HashMap::new();
    m.insert('A', 'E');
    m.insert('B', 'J');
    m.insert('C', 'K');
    m.insert('D', 'Y');
    m.insert('E', 'A');
    m.insert('F', 'P');
    m.insert('G', 'H');
    m.insert('H', 'G');
    m.insert('I', 'L');
    m.insert('J', 'B');
    m.insert('K', 'C');
    m.insert('L', 'I');
    m.insert('M', 'Q');
    m.insert('N', 'T');
    m.insert('O', 'S');
    m.insert('P', 'F');
    m.insert('Q', 'M');
    m.insert('R', 'W');
    m.insert('S', 'O');
    m.insert('T', 'N');
    m.insert('U', 'V');
    m.insert('V', 'U');
    m.insert('W', 'R');
    m.insert('X', 'Z');
    m.insert('Y', 'D');
    m.insert('Z', 'X');
    reflector{
        mapping: m
    }
    
}

impl plugboard {
    pub fn map(&self, input: char) -> char {
        for w in &self.plugs{
            if w.0 == input {
                return w.1;
            }
        }
        return '_';
    }
}

/// create_plugboard reads the plugboard configuration in the provided file
pub fn create_plugboard(filename: String) -> plugboard {
    let input = fs::read_to_string(filename).expect("could not read file");
    let lines: Vec<String> = input.to_uppercase().split("\n").map(str::to_string).collect();
    let base = lines[0].split(" ");
    let map = lines[1].split(" ");

    let mut pins: Vec<pins> = vec!();
    let mut idx = 0;
    let mut static_rotor_map : HashMap<char, u8> = HashMap::new();
    for i in base.zip(map) {
        let ci = i.0.chars().next().unwrap();
        let ci2 = i.1.chars().next().unwrap();
        pins.push((ci, ci2));
        static_rotor_map.insert(ci2, idx);
        idx += 1;
    }

    plugboard{plugs: pins, output_to_static_rotor: static_rotor_map}
}

pub struct enigma_machine {
    pub plugboard: plugboard,
    pub reflector: reflector,
    pub rotors: Vec<rotors::rotor>
}


impl enigma_machine {

    pub fn encrypt(&mut self, input: String) -> String {
        let chars: Vec<char> = input.to_uppercase().chars().collect();

        let mut output: Vec<char> = vec!();
        // send each char through the rotors
        for c in chars {
            // first we send it through the plugboard.. 
            let pc = self.plugboard.map(c);
            println!("plugboard {} --> {}", c, pc);
            let mut connector = self.plugboard.output_to_static_rotor.get(&pc).unwrap();
            //println!("static output pin {} --> {}", pc, connector);

            let mut result = pc;
            for rotor in &mut self.rotors {
                let current_char = rotor.get_input_at_pin(*connector);
                result = rotor.map(current_char);
                connector = rotor.output_pin_position.get(&result).unwrap();
            }
            // println!("mapped {} to char: {}", c, result);

            // todo: implement reflector! 
            result = *self.reflector.mapping.get(&result).unwrap();
            // calculate the pin at which result enters.. 
            let tmp = tools::char_to_idx(result);
            connector = &tmp;

            println!("result before reflector-passthrough: {}", result);
            for n in (0..self.rotors.len()).rev() {
                let rotor = &self.rotors[n];
                let current_char = rotor.get_output_at_pin(*connector);
                result = rotor.reflector_map(current_char);
                //println!("char at current output: {} connects to {}", current_char, result);
                connector = rotor.output_pin_position.get(&result).unwrap();
                println!("{}", connector);
            }
            println!("result after reflector-passthrough: {}", result);

            output.push(result.clone());
        }



        output.into_iter().collect()
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
    let pboard = create_plugboard("src/plug2.board".to_string());
    enigma_machine{plugboard: pboard, rotors: rs, reflector: create_reflector()}
}
