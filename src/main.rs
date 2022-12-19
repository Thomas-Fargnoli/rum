
use rum::input::load;
use std::env;
use rum::rum::{Rum};


fn main() {
    let input = env::args().nth(1);
    let mut rum_sys: Rum = Rum{registers: [0; 8], memory_segments: Vec::new(), program_counter: 0, empty_keys: vec![]};
    rum_sys.memory_segments.push(load(input.as_deref()));
    loop{
        let inst = rum_sys.memory_segments[0][rum_sys.program_counter as usize];
        rum_sys.run(inst);
    }
}

