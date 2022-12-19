
use crate::instructions::{
    conditional_move, 
    load_memory, 
    store_memory,
    addition,
    multiplication,
    division,
    nand,
    halt,
    output,
    input,
    load_prog,
    val_load
};
use crate::mapping::{
    create_memory,
    unmap_memory
};

enum Opcode {
    CMov,         // 0
    Load,         // 1
    Store,        // 2
    Add,          // 3
    Mul,          // 4
    Div,          // 5
    Nand,         // 6
    Halt,         // 7
    MapSegment,   // 8
    UnmapSegment, // 9
    Output,       // 10
    Input,        // 11
    LoadProgram,  // 12
    LoadValue,    // 13
}

pub struct Field {
    width: u32,
    lsb: u32,
}

static RA: Field = Field { width: 3, lsb: 6 };
static RB: Field = Field { width: 3, lsb: 3 };
static RC: Field = Field { width: 3, lsb: 0 };
static RL: Field = Field {width: 3, lsb: 25};
static VL: Field = Field {width: 25, lsb: 0};
static OP: Field = Field { width: 4, lsb: 28 };

fn mask(bits: u32) -> u32 {
    (1 << bits) - 1
}

pub fn get(field: &Field, instruction: u32) -> u32 {
    (instruction >> field.lsb) & mask(field.width)
}

// All of the code above this point was provided via the rumdump lab

/// This struct holds that values need to run a universal machine within rust
/// 
/// # fields:
/// 'registers': an array of 8 u32 values
/// 'memory_segments': a 2d vector of u32's
/// 'program_counter': a u32 value
/// 'empty_keys': a vector of u32's
pub struct Rum {
    pub registers: [u32; 8],
    pub memory_segments: Vec<Vec<u32>>,
    pub program_counter: u32,
    pub empty_keys: Vec<u32>
}

impl Rum {

    /// This method will run the given instruction and update the rum struct based on
    /// the instruction that is input into the method
    /// 
    /// # Arguments:
    /// 'inst': a 'word' stored as a u32
    pub fn run(&mut self, inst: u32){
        let reg_a = get(&RA, inst);
        let reg_b = get(&RB, inst);
        let reg_c = get(&RC, inst);
        match get(&OP, inst){
            o if o == Opcode::CMov as u32 => {
                conditional_move(self, reg_a, reg_b, reg_c);
                self.program_counter += 1;
            }
            o if o == Opcode::Load as u32 => {
                load_memory(self, reg_a, reg_b, reg_c);
                self.program_counter += 1;
            }
            o if o == Opcode::Store as u32 => {
                store_memory(self, reg_a, reg_b, reg_c);
                self.program_counter += 1;
            }
            o if o == Opcode::Add as u32 => {
                addition(self, reg_a, reg_b, reg_c);
                self.program_counter += 1;
            }
            o if o == Opcode::Mul as u32 => {
                multiplication(self, reg_a, reg_b, reg_c);
                self.program_counter += 1;
            }
            o if o == Opcode::Div as u32 => {
                division(self, reg_a, reg_b, reg_c);
                self.program_counter += 1;
            }
            o if o == Opcode::Nand as u32 => {
                nand(self, reg_a, reg_b, reg_c);
                self.program_counter += 1;
            }
            o if o == Opcode::Halt as u32 => {
                halt();
            }
            o if o == Opcode::MapSegment as u32 => {
                create_memory(self, reg_b, reg_c);
                self.program_counter += 1;
            }
            o if o == Opcode::UnmapSegment as u32 => {
                unmap_memory(self, reg_c);
                self.program_counter += 1;
            }
            o if o == Opcode::Output as u32 => {
                output(self, reg_c);
                self.program_counter += 1;
            }
            o if o == Opcode::Input as u32 => {
                input(self, reg_c);
                self.program_counter += 1;
            }
            o if o == Opcode::LoadProgram as u32 => {
                load_prog(self, reg_c, reg_b);
            }
            o if o == Opcode::LoadValue as u32 => {
                val_load(self, get(&RL, inst),
                get(&VL, inst)
                );
                self.program_counter += 1;
            }
            _ => {
                panic!("This is not a proper instruction");
            }
        }
    }
}

