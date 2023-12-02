use cardiac_core::Assembler;
use std::collections::HashMap;

fn main() {
    // Create a new Assembler instance
    let mut assembler: Assembler = Assembler::new();

    // Load a `sum two nums` program
    let code: HashMap<u32, i32> = HashMap::from([
        (22, 100),
        (23, 410),
        (24, 644),
        (25, 144),
        (26, 544),
        (27, 700),
        (28, 330),
        (29, 824),
        (30, 900),
    ]);
    assembler.load_program(code);

    // Set target in program init
    assembler.set_target(22);

    // Ejecute the program
    while assembler.check_run() {
        assembler.next_step();
    }
}
