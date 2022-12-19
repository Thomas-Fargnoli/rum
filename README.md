Thomas Fargnoli &  Michaela Healy

Analyzing: 10 hours

Design Prep: 15 hours

Solving: 20 hours

Collaboration:
    Issac Chen
    Vincent Zhuang
    Christian Tropeano
    Larry Khan
    Connor Gray

Acknowledgements:
    As far as our implementation we feel that everything is implemented correctly. However, we feel that with more time, maybe our modulization could have been cleaner and our program could have been faster.

Departures:
    Our design changed quite significantly, as we chose to do a 2d vector as opposed to a hasmap when storing memory. We also decided to place our universal machine into a struct as opposed to being stored in main.

Architecture:
Input.rs - This file will contain the code we used in the disassembler of our rumdump lab. This will allow us to load in all the instructions from a um file and set them to our memory segment at 0.
    load(input: Option<&str>) - This particular function takes in an option string, which is used to determine if we have a filename or will read from stdin. After determining this, we read our instructions and create a vector of u32 words, which we will return. This will be mapped to our 0 index in memory.

Main.rs - Initialize our rum struct and begin the loop that will run our rum struct method called run()

Instructions.rs - This particular file will contain the functions we will call in the opcode match statement, each of which will interact with either segments, registers, or both to perform specific operations to simulate a machine.
    Conditional_Move(Machine:&mut Rum, Reg_A: u32, Reg_C: u32, Reg_B: u32) - This function will copy register B to register A only if our register C is not equal to the value 0. This will take in the three registers and will make changes to the register without returning a value.

    Load(Machine:&mut Rum,Reg_A: u32, Reg_C: u32, Reg_B: u32) - This function will load a memory segment at index register B, from instruction number register C, and set register A to the value found at that particular register, this will hold no return as we are simply editing our memory and registers directly

    Store(Machine:&mut Rum,Reg_A: u32, Reg_C: u32, Reg_B: u32) - This function will store a value from register C into a memory segment at register A on instruction register B. There will be no return, just edits to our memory and registers.

    Addition(Machine:&mut Rum,Reg_A: u32, Reg_C: u32, Reg_B: u32) - This function will take in all three registers. Register B and Register C, add them together, mod them by 2 ^ 32, then set Register A to this new value. There will be no return, just edits to our memory and registers.

    Multiplication(Machine:&mut Rum,Reg_A: u32, Reg_C: u32, Reg_B: u32) -  This function will take in all three registers. Register B and Register C, multiply them together, mod them by 2 ^ 32, then set Register A to this new value. There will be no return, just edits to our memory and registers.

    Division(Machine:&mut Rum,Reg_A: u32, Reg_C: u32, Reg_B: u32) -  This function will take in all three registers. Register B and Register C, divide them, mod them by 2 ^ 32, then set Register A to this new value. There will be no return, just edits to our memory and registers.

    NAND(Machine:&mut Rum,Reg_A: u32, Reg_C: u32, Reg_B: u32) - Determines if register B and register C are equal and takes the not of that, then sets that to register A. There will be no return, just edits to our memory and registers.

    Halt() - When halt is called this will end our program/machines, so this command does not take in or return anything and just ends the machines run.

    Output(Machine:&mut Rum, Reg_C: u32) - This function takes in the register C and outputs the value of that register, throws an error if the value is not between 0 and 255. This is checked in input, but we will also put a check here to account for possible errors from computations being performed. There will be no return as this outputs our values.

    Input(Machine:&mut Rum, Reg_C: u32) - This function takes in register C and waits for input. Once input is received, said input is loaded into register C. This value should be a 32-bit word and fall between the values 0 and 255. There will be no return, just edits to our memory and registers.

    Load_Prog(Machine:&mut Rum, Reg_C: u32, Reg_B: u32) - This function takes in registers C and B, as well as our machine. It will first duplicate the memory segment found at register B’s value. It will then clear memory segment 0 and replace it with the duplicated memory segment. The program counter is taken in so that we can jump to the instruction found at register C’s value in our new memory segment 0.

    Val_Load(Machine:&mut Rum, Reg_A: u32, word:u32)  - This function takes a word, and gets 25 bits from a 32-bit word. The function will determine the value and load it into register A. This will not return a value as we are loading the values into our registers.

Mapping.rs - We chose to separate mapping and unmapping memory into its own file due to our desire to add a way to determine capacity, and open the options to add more possible functions. This file contains changes to memory, such as adding segments to our memory vector or removing segments from the memory vector.

    Create_Memory(Machine:&mut Rum, Reg_C: u32, Reg_B: u32) - This function adds a memory segment to our 2d memory vector of memory segment. This is done by pushing a vector with an amount of words equal to register C’s values. Inside this function we will check if register B has a pattern and if it does, we add this memory segment. If it does not exist then we add to our memory vector a vector of 0’s equal to the size of register C.

    Unmap_memory(Machine:&mut Rum, Reg_C: u32) - This function goes through the memory vector and removes the segment mapped to the memory segment at register C’s value. This returns nothing as it makes changes to our rum machine

rum.rs - This file contains all the elements that build up the ground of our rum machine, such as registers, opcode identification, or bit fields
    rumdump lab materials - This file contains most of the disassembler from our rumdump lab, such as the enum, field struct, registers using field struct, and the conditionals
    Rum struct - This contains our registers, memory segments, empty memory segments, and our program counter. This contains a run method that runs a single instruction in the um.

Estimated 50 Million:
    To find out 50 million instructions estimation, we decided to time midmark as it holds 80 million instructions. We then determined the percentage that 50 million was of 80 million and applied that to the 15 seconds it took to run midmark. After calculations, our result was 9.375 seconds to complete 50 million instructions
    
    Final Result: 9.375 seconds