use std::collections::HashMap;

/// Emulator of the **CARDIAC** cardboard computer.  
/// info in [wikipedia](wikipedia.org/wiki/CARDboard_Illustrative_Aid_to_Computation)
/// 
/// Simulator for tests: [simulator](https://www.cs.drexel.edu/~bls96/museum/cardsim.html)
pub struct Assembler {
    memory: [i32; 100],
    accumulator: i32,
    target: u32,
    flag: bool,
    step: i32,
    input_deck: Vec<i32>,
    output_deck: Vec<i32>,
    instruction_map: HashMap<u32, fn(&mut Self, usize)>,
    run: bool,
}

impl Assembler {
    /// Creates an new `Assembler`
    pub fn new() -> Self {
        let mut instruction_map: HashMap<u32, fn(&mut Self, usize)> = HashMap::new();
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

        let mut memory: [i32; 100] = [0; 100];
        memory[0] = 1;
        memory[99] = 800;

        Self {
            memory: memory,
            accumulator: 0,
            target: 0,
            flag: true,
            step: 0,
            input_deck: Vec::new(),
            output_deck: Vec::new(),
            instruction_map: instruction_map,
            run: true,
        }
    }

    /// Take a number from the input card ("INPut") and place it in the specified memory cell.
    fn inp(&mut self, address: usize) {
        if let Some(value) = self.input_deck.pop() {
            if !(-999 <= value && value <= 999) {
                panic!("Value in input card out of range: {}", value);
            }
            self.memory[address] = value;
        } else {
            self.memory[address] = 0;
        }
    }

    /// Clear the accumulator and add the contents of a memory cell to the accumulator.
    fn cla(&mut self, address: usize) {
        let value: i32 = self.memory[address];
        self.accumulator = value;
    
        // Check accumulator sign
        self.flag = value >= 0;
    }

    /// Add the contents of a memory cell to the accumulator.
    fn add(&mut self, address: usize) {
        let value: i32 = self.memory[address];
        self.accumulator += value;

        // Check accumulator sign
        self.flag = value >= 0;
    }

    /// Performs a sign test on the contents of the accumulator.
    /// if minus, jump to a specified memory cell.
    fn tac(&mut self, address: usize) {
        if self.flag {
            self.jmp(address);
        }
    }

    /// Shifts the accumulator x places left, then y places right, where x is the upper address digit and y is the lower.
    fn sft(&mut self, address: usize) {
        let upper_digit = address / 10;
        let lower_digit = address % 10;
    
        // Shift left by x places
        self.accumulator <<= upper_digit;
    
        // Scroll right by and places
        self.accumulator >>= lower_digit;
    }

    /// Take a number from the specified memory cell and write it on the output card.
    fn out(&mut self, address: usize) {
        let value: i32 = self.memory[address];
        self.output_deck.push(value);
    }

    /// Copy the contents of the accumulator into a specified memory cell.
    fn sto(&mut self, address: usize) {
        let last_three_digits = self.accumulator % 1000;
        self.memory[address] = last_three_digits;
    }

    /// Subtract the contents of a specified memory cell from the accumulator.
    fn sub(&mut self, address: usize) {
        let value: i32 = self.memory[address];
        self.accumulator -= value;
        self.flag = value >= 0;
    }

    /// Jump to a specified memory cell. 
    /// The current cell number is written in cell 99. 
    /// This allows for one level of subroutines by having the return be the instruction at cell 99 (which had '8' hardcoded as the first digit.
    fn jmp(&mut self, address: usize) {
        self.memory[99] = format!("8{:02}", self.target).parse().unwrap();
        self.target = address as u32;
    }

    /// Move the program counter to the specified cell and stop program execution ("Halt and ReSet").
    fn hrs(&mut self, address: usize) {
        self.target = address as u32;
        self.run = false;
    }

    pub fn check_run(&self) -> bool {
        self.run
    }

    pub fn reset(&mut self) {
        self.target = 0;
        self.step = 0;
        self.accumulator = 0;
        self.input_deck = Vec::new();
        self.output_deck = Vec::new();
        self.flag = true;
        self.run = true;
    }

    pub fn clear_memory(&mut self) {
        self.memory = [0; 100];
        self.memory[0] = 1;
        self.memory[99] = 800;
    }

    pub fn update_cell(&mut self, address: u32, instruction: i32) {
        self.memory[address as usize] = instruction;
    }

    pub fn load_program(&mut self, program: HashMap<u32, i32>) {
        for (address, instruction) in program {
            self.memory[address as usize] = instruction;
            // println!("Address {address}; Instruction {instruction};")
        } 
    }

    pub fn next_step(&mut self) {
        self.run = true;

        let instruction: i32 = self.memory[self.target as usize];

        let opcode: u32 = (instruction / 100) as u32;
        let address: u32 = (instruction % 100) as u32;
        
        if let Some(instruction_fn) = self.instruction_map.get(&opcode) {
            instruction_fn(self, address as usize);
            self.step += 1;
        } else {
            panic!("Opcode undefined: {}", opcode);
        }
    }

}