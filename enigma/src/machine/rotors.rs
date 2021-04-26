type rotor_pins = (char, char);

pub struct rotor {
    // rotor configuration
    pub id: String,
    pub wiring: Vec<rotor_pins> ,
    pub position: u8 // current rotor position (0->26)
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

pub fn create_rotor(name: String, input: Vec<char>) -> rotor {
    let rotor_wiring = create_rotor_wiring(input);
    rotor {
        id: name,
        wiring: rotor_wiring,
        position: 0
    }
}


impl rotor {

    pub fn set_pos(&mut self, new_pos: u8) {
        self.position = new_pos;
    }

    pub fn incr_pos(&mut self) {
        self.position += 1;
    }

    pub fn map(&self, input: char) -> char {
        for w in &self.wiring {
            if w.0 == input {
                return w.1;
            }
        }
        return 'c';
    }

}
