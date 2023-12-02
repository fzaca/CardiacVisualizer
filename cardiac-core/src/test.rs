#[cfg(test)]
use super::*;

#[test]
fn test_assembler_hello_world() {
    // Create a new Assembler instance
    let mut assembler: Assembler = Assembler::new();

    // Load a `hello world` program
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

    let result: &Vec<i32> = assembler.get_output_card();
    let expected_result: &Vec<i32> = &Vec::from([10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);

    assert_eq!(result, expected_result)
}

#[test]
fn test_assembler_sum_two_nums() {
    // Create a new Assembler instance
    let mut assembler: Assembler = Assembler::new();

    // Load a `sum two nums` program
    let code: HashMap<u32, i32> = HashMap::from([
        (10, 3),
        (11, 004),
        (12, 103),
        (13, 204),
        (14, 605),
        (15, 505),
        (16, 900),
    ]);
    assembler.load_program(code);

    // Set target in program init
    assembler.set_target(10);

    // Load values in input card
    assembler.add_input(5);
    assembler.add_input(8);

    // Ejecute the program
    while assembler.check_run() {
        assembler.next_step();
    }

    let result: &Vec<i32> = assembler.get_output_card();
    let expected_result: &Vec<i32> = &Vec::from([13]);

    assert_eq!(result, expected_result)
}