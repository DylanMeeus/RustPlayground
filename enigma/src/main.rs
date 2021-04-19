use enigma::{create_plugboard, rotor};

#[allow(non_camel_case_types)]
#[allow(dead_code)]
mod enigma {

    use std::fs;
    pub struct rotor {
        // rotor configuration
        pub id: String
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
        plugboard: plugboard,
        rotors: Vec<rotor>
    }

    /*
    pub fn create_machine() -> enigma_machine {
        return enigma_machine{}
    }
    */

}

fn main() {

    let r = enigma::rotor{id: "hello".to_string()};
    println!("{:?}", r.id);

    enigma::create_plugboard("plug.board".to_string());

}
