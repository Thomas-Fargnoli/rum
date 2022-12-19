pub mod input;
pub mod instructions;
pub mod mapping;
pub mod rum;

#[cfg(test)]
mod tests {
    use crate::rum::{Rum};
    
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

//For whole program testing, we used the rum-binaries provides by the professor

#[test]
fn cond_mov(){
    let mut machine: &mut Rum =  &mut Rum {
        registers:[0,0,0,0,2,1,3,4],
        memory_segments:vec![vec![]],
        program_counter: 0 ,
        empty_keys: vec![]
    };

    conditional_move(machine,0, 4, 5);
    assert_eq!([2,0,0,0,2,1,3,4], machine.registers);

    conditional_move(machine, 1, 7, 2);
    assert_eq!([2,0,0,0,2,1,3,4], machine.registers);
}

#[test]
#[should_panic]
fn load_test_fail(){
    let mut machine: &mut Rum =  &mut Rum {
        registers:[0,0,0,0,2,1,3,4],
        memory_segments:vec![vec![], vec![0,2,1,3]],
        program_counter: 0 ,
        empty_keys: vec![]
    };
    load_memory(machine,0, 1, 5);
}

#[test]
fn load_test(){
    let mut machine: &mut Rum =  &mut Rum {
        registers:[0,0,0,0,2,1,3,4],
        memory_segments:vec![vec![], vec![0,2,1,3]],
        program_counter: 0 ,
        empty_keys: vec![]
    };
    load_memory(machine, 0, 5, 6);
    assert_eq!([3,0,0,0,2,1,3,4], machine.registers);
}

#[test]
#[should_panic]
fn store_fail_test(){
    let mut machine: &mut Rum =  &mut Rum {
        registers:[0,0,0,0,2,1,3,4],
        memory_segments:vec![vec![], vec![0,2,1,3]],
        program_counter: 0 ,
        empty_keys: vec![]
    };
    store_memory(machine, 0, 5, 5);
    
}

#[test]
fn store_test(){
    let mut machine: &mut Rum =  &mut Rum {
        registers:[0,0,0,0,2,1,3,4],
        memory_segments:vec![vec![], vec![0,2,1,3]],
        program_counter: 0 ,
        empty_keys: vec![]
    };
    store_memory(machine, 5, 5, 7);
    assert_eq!(vec![0,4,1,3], machine.memory_segments[1]);
}

#[test]
fn add_test(){
    let mut machine: &mut Rum =  &mut Rum {
        registers:[0,0,0,0,2,1,3,4],
        memory_segments:vec![vec![]],
        program_counter: 0 ,
        empty_keys: vec![]
    };
    addition(machine, 0, 5, 7);
    assert_eq!([5,0,0,0,2,1,3,4], machine.registers);
}

#[test]
fn mul_test(){
    let mut machine: &mut Rum =  &mut Rum {
        registers:[0,0,0,0,2,1,3,4],
        memory_segments:vec![vec![]],
        program_counter: 0 ,
        empty_keys: vec![]
    };
    multiplication(machine, 0, 6, 7);
    assert_eq!([12,0,0,0,2,1,3,4], machine.registers);
}

#[test]
fn div_test(){
    let mut machine: &mut Rum =  &mut Rum {
        registers:[0,0,0,0,2,1,3,4],
        memory_segments:vec![vec![]],
        program_counter: 0 ,
        empty_keys: vec![]
    };
    division(machine, 0, 7, 4);
    assert_eq!([2,0,0,0,2,1,3,4], machine.registers);
}

#[test]
fn nand_test(){
    let mut machine: &mut Rum =  &mut Rum {
        registers:[0,0,0,0,2,1,3,4],
        memory_segments:vec![vec![]],
        program_counter: 0 ,
        empty_keys: vec![]
    };
    division(machine, 0, 7, 4);
    assert_eq!([2,0,0,0,2,1,3,4], machine.registers);
}


#[test]
fn map_test(){
    let mut machine: &mut Rum =  &mut Rum {
        registers:[0,0,0,0,2,1,3,4],
        memory_segments:vec![vec![1,2,3,4,5,6], vec![]],
        program_counter: 0 ,
        empty_keys: vec![1]
    };
    create_memory(machine, 2, 4);
    let temp_vec: Vec<u32> = vec![];
    assert_eq!(temp_vec, machine.empty_keys);
    assert_eq!(vec![0,0], machine.memory_segments[1]);
    assert_eq!(1, machine.registers[2]);
    
    create_memory(machine, 0, 7);
    assert_eq!(vec![0,0,0,0], machine.memory_segments[2]);
    assert_eq!(2, machine.registers[0]);
}

#[test]
fn unmap_test(){
    let mut machine: &mut Rum =  &mut Rum {
        registers:[0,0,0,0,2,1,3,4],
        memory_segments:vec![vec![], vec![1,2,3,4,5,6]],
        program_counter: 0 ,
        empty_keys: vec![]
    };

    unmap_memory(machine,5);
    assert_eq!(vec![1], machine.empty_keys);
    let temp_mem:Vec<Vec<u32>> = vec![vec![], vec![]];
    assert_eq!(temp_mem, machine.memory_segments);
    
}

#[test]
#[should_panic]
fn output_test(){
    let mut machine: &mut Rum =  &mut Rum {
        registers:[256,0,0,0,2,1,3,4],
        memory_segments:vec![vec![]],
        program_counter: 0 ,
        empty_keys: vec![]
    };
    output(machine, 0);
    
}

#[test]
fn load_prog_test(){
    let mut machine: &mut Rum =  &mut Rum {
        registers:[0,0,0,0,2,1,3,4],
        memory_segments:vec![vec![], vec![1,2,3]],
        program_counter: 0 ,
        empty_keys: vec![]
    };
    
    load_prog(machine, 4, 5);
    assert_eq!(2, machine.program_counter);
    let temp: Vec<u32> = vec![1,2,3];
    assert_eq!(vec![1,2,3], machine.memory_segments[0]);
}

#[test]
fn val_load_test(){
    let mut machine: &mut Rum =  &mut Rum {
        registers:[0,0,0,0,2,1,3,4],
        memory_segments:vec![vec![]],
        program_counter: 0 ,
        empty_keys: vec![]
    };
    val_load(machine, 0, 46);
    assert_eq!(46, machine.registers[0]);
}

}