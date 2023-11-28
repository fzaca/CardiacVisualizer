use std::collections::HashMap;

/// Emulator of the CARDIAC cardboard computer.
/// info in wikipedia.org/wiki/CARDboard_Illustrative_Aid_to_Computation
pub struct Assembler {
    memory: [u32; 100],
    accumulator: i32,
    target: u32,
    flag: bool,
    step: i32,
    input_deck: Vec<i32>,
    output_deck: Vec<i32>,
    instruction_map: HashMap<u8, fn(&Self, u32)>,
}

impl Assembler {
    /// Creates an new `Assembler`
    pub fn new() -> Self {
        let mut instruction_map: HashMap<u8, fn(&Self, u32)> = HashMap::new();
        instruction_map.insert(0, Self::inp);
        instruction_map.insert(0, Self::cla);
        instruction_map.insert(0, Self::add);
        instruction_map.insert(0, Self::tac);
        instruction_map.insert(0, Self::sft);
        instruction_map.insert(0, Self::out);
        instruction_map.insert(0, Self::sto);
        instruction_map.insert(0, Self::sub);
        instruction_map.insert(0, Self::jmp);
        instruction_map.insert(0, Self::hrs);

        Self {
            memory: [0; 100],
            accumulator: 0,
            target: 0,
            flag: true,
            step: 0,
            input_deck: Vec::new(),
            output_deck: Vec::new(),
            instruction_map: instruction_map,
        }
    }

    fn inp(&self, address: u32) {
        println!("inp {address}")
    }

    fn cla(&self, address: u32) {
        println!("inp {address}")
    }

    fn add(&self, address: u32) {
        println!("inp {address}")
    }

    fn tac(&self, address: u32) {
        println!("inp {address}")
    }

    fn sft(&self, address: u32) {
        println!("inp {address}")
    }

    fn out(&self, address: u32) {
        println!("inp {address}")
    }

    fn sto(&self, address: u32) {
        println!("inp {address}")
    }

    fn sub(&self, address: u32) {
        println!("inp {address}")
    }

    fn jmp(&self, address: u32) {
        println!("inp {address}")
    }

    fn hrs(&self, address: u32) {
        println!("inp {address}")
    }

}