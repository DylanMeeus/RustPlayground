use enigma::{create_plugboard, rotor};

#[allow(non_camel_case_types)]
#[allow(dead_code)]

mod enigma {
    use std::fs;

    type rotor_pins = (u8, u8);

    pub struct rotor {
        // rotor configuration
        pub id: String,
        pub pins: Vec<rotor_pins> 
    }

    pub struct plugboard {
        // plugboard configuration
    }

    /// create_plugboard reads the plugboard configuration in the provided file
    pub fn create_plugboard(filename: String) -> plugboard {
        let input = fs::read_to_string(filename).expect("could not read file");
        let lines: Vec<String> = input.split("\n").map(str::to_string).collect();
        let base = lines[0].split(" ");
        let map = lines[1].split(" ");
        for i in base.zip(map) {
            println!("({}, {})", i.0, i.1)
        }

        plugboard{}
    }

    pub struct enigma_machine {
        pub plugboard: plugboard,
        pub rotors: Vec<rotor>
    }

    pub fn create_machine() -> enigma_machine {
        let first_rotor = rotor {
            id: "A".to_string(),
            pins: vec![(1,2)]
        };

        let rs = vec![first_rotor];

        let pboard = create_plugboard("src/plug.board".to_string());
        enigma_machine{plugboard: pboard, rotors: rs}
    }

}

fn main() {
    let machine = enigma::create_machine();
    println!("{:?}", machine.rotors[0].pins[0]);
}
