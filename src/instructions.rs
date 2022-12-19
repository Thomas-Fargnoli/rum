use std::io::{stdin, Read};

use crate::rum::{Rum};

/// This function performs opcode 0, or the condition move which will set the value at register b to register a's value
/// 
/// # Arguments:
/// 'machine': A call of the RUM struct to hold the current version of the machine
/// 'reg_a': the index of register a stored as a u32
/// 'reg_b': the index of register b stored as a u32
/// 'reg_c': the index of register b stored as a u32
pub fn conditional_move(machine: &mut Rum, reg_a: u32, reg_b: u32, reg_c: u32){
    if machine.registers[reg_c as usize] != 0 {
        machine.registers[reg_a as usize] = machine.registers[reg_b as usize];
    }
}

/// This function performs opcode 1, where we load the value stored on memory segment
/// held at register b's index value, at register c's value index
/// The value index would be present in the vector being stored at the index of register b on our memory segments
/// 
/// # Arguments:
/// 'machine': A call of the RUM struct to hold the current version of the machine
/// 'reg_a': the index of register a stored as a u32
/// 'reg_b': the index of register b stored as a u32
/// 'reg_c': the index of register b stored as a u32
pub fn load_memory(machine: &mut Rum, reg_a: u32, reg_b: u32, reg_c: u32){
    if machine.memory_segments[machine.registers[reg_b as usize]as usize].is_empty(){
        panic!("Refers to unmapped memory")
    }
    machine.registers[reg_a as usize] = machine.memory_segments[machine.registers[reg_b as usize]as usize][machine.registers[reg_c as usize] as usize];
}

/// This function performs opcode 2, where we store the value of register c into the memory
/// segment at index register a, at index register b.
/// 
/// # Arguments:
/// 'machine': A call of the RUM struct to hold the current version of the machine
/// 'reg_a': the index of register a stored as a u32
/// 'reg_b': the index of register b stored as a u32
/// 'reg_c': the index of register b stored as a u32
pub fn store_memory(machine: &mut Rum, reg_a: u32, reg_b: u32, reg_c: u32){
    if machine.memory_segments[machine.registers[reg_a as usize]as usize].is_empty(){
        panic!("Refers to unmapped memory")
    }
    machine.memory_segments[machine.registers[reg_a as usize]as usize][machine.registers[reg_b as usize] as usize] = machine.registers[reg_c as usize];
    
}

/// This function performs opcode 3, which adds the value at register b together with the value at register c
/// and setting register a to this new value
/// 
/// # Arguments:
/// 'machine': A call of the RUM struct to hold the current version of the machine
/// 'reg_a': the index of register a stored as a u32
/// 'reg_b': the index of register b stored as a u32
/// 'reg_c': the index of register b stored as a u32
pub fn addition(machine: &mut Rum, reg_a: u32, reg_b: u32, reg_c: u32){
    machine.registers[reg_a as usize] = ((machine.registers[reg_b as usize] as u64 + machine.registers[reg_c as usize] as u64) % (2 as u64).pow(32)) as u32;
}

/// This function performs opcode 4, which multiplies the value at register b together with the value at register c
/// and setting register a to this new value
/// 
/// # Arguments:
/// 'machine': A call of the RUM struct to hold the current version of the machine
/// 'reg_a': the index of register a stored as a u32
/// 'reg_b': the index of register b stored as a u32
/// 'reg_c': the index of register b stored as a u32
pub fn multiplication(machine: &mut Rum, reg_a: u32, reg_b: u32, reg_c: u32){
    machine.registers[reg_a as usize] = ((machine.registers[reg_b as usize] as u64 * machine.registers[reg_c as usize] as u64 ) % (2 as u64).pow(32)) as u32;
}

/// This function performs opcode 5, which divides the value at register b by the value at register c
/// and setting register a to this new value
/// 
/// # Arguments:
/// 'machine': A call of the RUM struct to hold the current version of the machine
/// 'reg_a': the index of register a stored as a u32
/// 'reg_b': the index of register b stored as a u32
/// 'reg_c': the index of register b stored as a u32
pub fn division(machine: &mut Rum, reg_a: u32, reg_b: u32, reg_c: u32){
    machine.registers[reg_a as usize] = machine.registers[reg_b as usize] / machine.registers[reg_c as usize] ;
}

/// This function performs opcode 6, which nands the value at register b together with the value at register c
/// and setting register a to this new value
/// 
/// # Arguments:
/// 'machine': A call of the RUM struct to hold the current version of the machine
/// 'reg_a': the index of register a stored as a u32
/// 'reg_b': the index of register b stored as a u32
/// 'reg_c': the index of register b stored as a u32
pub fn nand(machine: &mut Rum, reg_a: u32, reg_b: u32, reg_c: u32){
    machine.registers[reg_a as usize] = !(machine.registers[reg_b as usize] & machine.registers[reg_c as usize] );
}

/// This function performs opcode 7, which ends out program and closes the machine.
/// 
pub fn halt(){
    std::process::exit(0)
}

/// This function performs opcode 10, which takes a single ascii character stored as a u32 in register c, and prints its
/// char counterpart
/// 
/// # Arguments:
/// 'machine': A call of the RUM struct to hold the current version of the machine
/// 'reg_c': the index of register b stored as a u32
pub fn output(machine: &mut Rum, reg_c: u32){
    if machine.registers[reg_c as usize]  > 255 {
        panic!("Value not within 0 and 255");
    }
    print!("{}", char::from_u32(machine.registers[reg_c as usize]).unwrap());
}

/// This function performs opcode 11, which takes in a single char as a u32 and assigns it to register c.
/// 
/// # Arguments:
/// 'machine': A call of the RUM struct to hold the current version of the machine
/// 'reg_c': the index of register b stored as a u32
pub fn input(machine: &mut Rum, reg_c: u32){
    let mut buffer: [u8; 1] = [0];

    let single_char = stdin().read(&mut buffer);

    machine.registers[reg_c as usize]  = match single_char {
        Ok(0) => u32::MAX,
        Ok(1) => buffer[0] as u32,
        _ => panic!("Inproper Input")
    }
    
}

/// This function performs opcode 12, which takes the memory segment stored at register b's value index, and
/// replaces memory segment 0 with this memory segment. We then initialize our program counter at the value of
/// register c.
/// 
/// # Arguments:
/// 'machine': A call of the RUM struct to hold the current version of the machine
/// 'reg_b': the index of register b stored as a u32
/// 'reg_c': the index of register b stored as a u32
pub fn load_prog(machine: &mut Rum, reg_c: u32, reg_b:u32){
    machine.memory_segments[0] = machine.memory_segments[machine.registers[reg_b as usize] as usize].clone();
    machine.program_counter = machine.registers[reg_c as usize];
}

/// This function performs opcode 13, which stores the value of the first 25 bits of the instruction
/// inside of register a
/// 
/// # Arguments:
/// 'machine': A call of the RUM struct to hold the current version of the machine
/// 'reg_a': the index of register a stored as a u32
/// 'value': the value of the current instructions first 25 bits
pub fn val_load(machine: &mut Rum, reg_a: u32, value: u32){
    machine.registers[reg_a as usize] = value;
}