use crate::rumdis::{get, OP,Opcode, RA, RB, RC,RL,VL};
use std::process::exit;
use num_traits::FromPrimitive;
use crate::opcodes::{cmov,load,store,add,mult,div,nand,halt,map,unmap,output,input,loadprogram,loadvalue};
#[derive(Debug, Default, Clone, PartialEq)]

/// Struct for the Universal Machine
/// Contains the registers, memory, free memory, program counter and count
/// Registers: Vector of 8 registers
/// Memory: Vector of vectors of u32
/// Free Memory: Vector of usize
pub struct UniversalMachine {

    pub registers: [u32;8], // Vector of registers
    
    pub memory: Vec<Vec<u32>>,

    pub free_memory: Vec<usize>,

    pub program_counter: usize,
    pub count: usize,
}
impl UniversalMachine {
    pub fn new(inst: &Vec<u32>) -> UniversalMachine {
        UniversalMachine{
            memory: vec![inst.clone()],
            free_memory: vec![],
            registers: [0; 8],
            program_counter: 0,
            count: 0,
        }
    }
}
/// implementation of execute instructions for the Universal Machine
/// calls the respective functions for each opcode
pub fn execute_instruction(machine: &mut UniversalMachine,current_instruction: u32) {

    let ra = get(&RA, current_instruction) as usize;
    let rb = get(&RB, current_instruction) as usize;
    let rc = get(&RC, current_instruction) as usize;
    let rl = get(&RL,current_instruction) as usize;
    let vl = get(&VL,current_instruction) as usize;

    match Opcode::from_u32(get(&OP, current_instruction)) {
        Some(Opcode::CMov) => cmov(machine, ra, rb, rc),
        Some(Opcode::Load) => load(machine, ra, rb, rc),
        Some(Opcode::Store) => store(machine, ra, rb, rc),
        Some(Opcode::Add) => add(machine, ra, rb, rc),
        Some(Opcode::Mult) => mult(machine, ra, rb, rc),
        Some(Opcode::Div) => div(machine, ra, rb, rc),
        Some(Opcode::NAND) => nand(machine, ra, rb, rc),
        Some(Opcode::Halt) => halt(),
        Some(Opcode::Map) => map(machine, rb, rc),
        Some(Opcode::Unmap) => unmap(machine, rc),
        Some(Opcode::Output) => output(machine, rc),
        Some(Opcode::Input) => input(machine, rc),
        Some(Opcode::LoadProgram) => loadprogram(machine, rb, rc),
        Some(Opcode::LoadValue) => loadvalue(machine, rl, vl),
        None => {
            // Handle unknown opcode
            exit(0)
        }
    }
            
}
/// implementation of execute for the Universal Machine
/// calls execute_instruction for each instruction in the memory
/// and increments the program counter
pub fn execute(machine: &mut UniversalMachine) {
    
    while let Some(instruction) = machine.memory[0].get(machine.program_counter) {
  
        machine.program_counter += 1;
        execute_instruction(machine,instruction.clone());
    }
   
}
