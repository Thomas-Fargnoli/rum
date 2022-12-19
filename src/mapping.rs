use crate::rum::Rum;

/// This function performs opcode 8, which checks for empty memory and either
/// creates a new memory segment if all memory is filled, or fills in a cleared
/// memory segment that exists within the machine
/// 
/// # Arguments:
/// 'machine': A call of the RUM struct to hold the current version of the machine
/// 'reg_b': the index of register b stored as a u32
/// 'reg_c': the index of register b stored as a u32
pub fn create_memory(machine: &mut Rum, reg_b: u32, reg_c: u32){
    if machine.empty_keys.is_empty(){
        machine.memory_segments.push(vec![0; machine.registers[reg_c as usize] as usize]);
        machine.registers[reg_b as usize] = (machine.memory_segments.len() - 1) as u32;
    } else{
        let index_num = machine.empty_keys.pop().unwrap();
        machine.memory_segments[index_num as usize] = vec![0; machine.registers[reg_c as usize] as usize];
        machine.registers[reg_b as usize] = index_num
    }
}


/// This function performs opcode 9, which pushes the empty index into a vector holding
/// memory segments that are empty. It then clears that memory segment index of all data.
/// 
/// # Arguments:
/// 'machine': A call of the RUM struct to hold the current version of the machine
/// 'reg_c': the index of register b stored as a u32
pub fn unmap_memory(machine: &mut Rum, reg_c: u32){
    machine.empty_keys.push(machine.registers[reg_c as usize]);
    machine.memory_segments[machine.registers[reg_c as usize] as usize].clear()
}