use std::collections::HashMap;

type rotor_pins = (char, char);

pub struct rotor {
    // rotor configuration
    pub id: String,
    pub wiring: Vec<rotor_pins> ,
    pub position: u8, // current rotor position (0->26)

    pub input_pin_position: HashMap<char, u8>,
    pub output_pin_position: HashMap<char, u8>
}



/// create_rotor_wiring assumes wiring each element in "input" against the alphabet from
/// A-Z
fn create_rotor_wiring(input: Vec<char>) -> Vec<rotor_pins> {
    let mut idx = 0;
    let alph = vec!['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];

    input.into_iter()
        .map(|x| { 
            let r = (alph[idx], x);
            idx += 1;
            r
        }).
        collect()
}

pub fn create_input_output_maps(rp: Vec<rotor_pins>) -> (HashMap<char,u8>, HashMap<char,u8>)  {
    let mut input_map : HashMap<char, u8> = HashMap::new();
    let mut output_map : HashMap<char, u8> = HashMap::new();

    for (idx, tuple) in rp.iter().enumerate() {
        input_map.insert(tuple.0, idx as u8);
        output_map.insert(tuple.0, idx as u8);
    }


    (input_map, output_map)

}

pub fn create_rotor(name: String, input: Vec<char>) -> rotor {
    let rotor_wiring = create_rotor_wiring(input);
    
    let (input,output) = create_input_output_maps(rotor_wiring.clone());

    rotor {
        id: name,
        wiring: rotor_wiring,
        position: 0,
        input_pin_position: input,
        output_pin_position: output
    }
}


impl rotor {

    pub fn set_pos(&mut self, new_pos: u8) {
        self.position = new_pos;
    }

    /// incr_pos returns true if the position was reset, otherwise false
    /// TODO: do this smarter
    pub fn incr_pos(&mut self) -> bool {
        // todo: this only works for rotors that start in the default configuration for now
        self.position += 1;
        self.incr_pin_positions();
        if self.position >= 25 {
            self.position = 0;
            return true;
        }
        return false;
    }

    pub fn incr_pin_positions(&mut self) {
        fn incr(pins: &mut HashMap<char, u8>) {
            for val in pins.values_mut() {
                if *val != 25 {
                    *val += 1;
                } else {
                    *val = 0;
                }
            }
        }
        incr(&mut self.input_pin_position);
        incr(&mut self.output_pin_position);
    }
    

    pub fn get_input_at_pin(&self, pin: u8) -> char {
        self.get_char_at_pin(pin, &self.input_pin_position)
    }

    pub fn get_output_at_pin(&self, pin: u8) -> char {
        self.get_char_at_pin(pin, &self.output_pin_position)
    }

    fn get_char_at_pin(&self, pin: u8, pins : &HashMap<char, u8>) -> char {
        for (k,v) in pins {
            if *v == pin {
                return *k;
            }
        }
        panic!("Char not found");
    }

    /// reflector_map is the mapping after passing through the reflector
    pub fn reflector_map(&self, output_char: char) -> char {
        for w in &self.wiring {
            if w.1 == output_char {
                return w.0;
            }
        }
        panic!("char not found");
    }

    pub fn map(&self, input: char) -> char {
        for w in &self.wiring {
            if w.0 == input {
                return w.1;
            }
        }
        panic!("char not found");
    }

}
