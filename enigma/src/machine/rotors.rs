type rotor_pins = (u8, u8);
pub struct rotor {
    // rotor configuration
    pub id: String,
    pub wiring: Vec<rotor_pins> ,
    pub position: u8
}



/// create_rotor_wiring assumes wiring each element in "input" against the alphabet from
/// A-Z
fn create_rotor_wiring(input: Vec<char>) -> Vec<rotor_pins> {
    let mut idx = 0;
    input.into_iter()
        .map(|x| { idx += 1; (idx, x as u8) }).
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
