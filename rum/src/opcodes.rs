use std::io::{Read, Write};
use std::process::exit;
use std::io::{stdout,stdin};

//use crate::rumdis::{get, op, Opcode, RA, RB, RC};
use crate::execution::UniversalMachine;
/// Implementation for Conditional Move
/// $r[A] := $r[B] if $r[C] != 0
/// The register $r[A] is set to the value of $r[B], unless the value of $r[C] is 0.
/// The operation takes one instruction slot.
pub fn cmov(machine: &mut UniversalMachine, ra: usize, rb: usize, rc: usize) {
    if machine.registers[rc] != 0 {
        machine.registers[ra] = machine.registers[rb];
    }
}
/// Implementation for Segmented Load
/// $r[A] := $m[$r[B]][$r[C]]
/// The value $m[$r[B]][$r[C]] is loaded, even if it is not a valid
pub fn load(machine: &mut UniversalMachine, ra: usize, rb: usize,rc: usize) {
    let i_rb = machine.registers[rb] as usize;
    let i_rc = machine.registers[rc] as usize;
    machine.registers[ra] = machine.memory[i_rb][i_rc];
    
}
/// Implementation for Segmented Store
/// $m[$r[A]][$r[B]] := $r[C]
/// The value $r[C] is stored, even if it is not a valid
pub fn store(machine: &mut UniversalMachine, ra: usize, rb: usize, rc: usize) {
    let i_ra = machine.registers[ra] as usize;
    let i_rb = machine.registers[rb] as usize;
    machine.memory[i_ra][i_rb] = machine.registers[rc]; 
}
/// Implementation for Addition
/// $r[A] := ($r[B] + $r[C]) mod 2^32
/// The addition is performed modulo 2^32.
/// This instruction, when used in conjunction with the conditional move instruction,
pub fn add(machine: &mut UniversalMachine, ra: usize, rb: usize,rc: usize) {
    // Implementation for Addition
    // $r[A] := ($r[B] + $r[C]) mod 2^32
    let i_rc = machine.registers[rc] ;
    let i_rb = machine.registers[rb] ;

    machine.registers[ra] = i_rb.wrapping_add(i_rc); // handles overflow
   
}
/// Implementation for Multiplication
/// $r[A] := ($r[B] × $r[C]) mod 2^32
/// The upper 32 bits of the product $r[B] × $r[C] are discarded.
/// The lower 32 bits of the product $r[B] × $r[C] are placed in $r[A].
pub fn mult(machine: &mut UniversalMachine, ra: usize, rb: usize,rc: usize) {
    // Implementation for Addition
    // $r[A] := ($r[B] + $r[C]) mod 2^32
    let i_rc = machine.registers[rc];
    let i_rb = machine.registers[rb];


    machine.registers[ra] = i_rb.wrapping_mul(i_rc); // handles overflow

}
/// Implementation for Division
/// $r[A] := ($r[B] ÷ $r[C]) (integer division)
/// If $r[C] = 0, then the result is undefined.
pub fn div(machine: &mut UniversalMachine, ra: usize, rb: usize, rc: usize) {
    // Implementation for Division
    // $r[A] := ($r[B] ÷ $r[C]) (integer division)
    let i_rc = machine.registers[rc];
    let i_rb = machine.registers[rb];
   
    machine.registers[ra] = i_rb.wrapping_div(i_rc);

}
///
/// Implementation for Bitwise NAND
/// $r[A] := ¬($r[B] ∧ $r[C])
pub fn nand(machine: &mut UniversalMachine, ra: usize, rb: usize, rc: usize) {
  
    let i_rc = machine.registers[rc];
    let i_rb = machine.registers[rb];
    
    machine.registers[ra] = !(i_rb & i_rc);
   
}

pub fn halt() {
    
    exit(0); // Exit the execution loop
}
/// Implementation for Map segment
/// The segment $m[$r[C]] is duplicated, and the duplicate replaces $m[0].
/// The program counter is set to point to $m[0][$r[D]].
/// The new segment has the same initial contents as $m[$r[C]].
/// The new segment is unmapped.
pub fn map(machine: &mut UniversalMachine, rb: usize, rc: usize) {
   
    let num = machine.registers[rc] as usize;
   

    //get bit pattern that is not being used
    machine.registers[rb] = match machine.free_memory.pop() {
        Some(segment_id) => segment_id as u32,
        None => machine.memory.len() as u32,
    };

    let val = machine.registers[rb] as usize;
    if machine.memory.len() == val {
        machine.memory.push(vec![0; num]);
    }else{
        machine.memory[val].resize(num, 0);
    }

   
}

 /// Implementation for Unmap segment
 /// The segment $m[$r[C]] is unmapped.
 /// Future Map Segment instructions may reuse the identifier $r[C].
/// The words in the unmapped segment are made available for reuse

pub fn unmap(machine: &mut UniversalMachine, rc: usize) {
   

    let val = machine.registers[rc] as usize;

    machine.memory[val].clear();

    machine.free_memory.push(val);
}

/// Output a character  $r[C] to the I/O device 
/// specified by $r[B].
/// If $r[C] is 0, then the I/O device immediately
/// flushes its output buffer and waits until all
pub fn output(machine: &mut UniversalMachine, rc: usize) {
    let val = machine.registers[rc] as u8;
    let buffer = [val; 1];
    if let Err(e) = stdout().write(&buffer).and_then(|_| stdout().flush()) {
        eprintln!("Error writing to stdout: {}", e);
    }
}
/// Input a character to the I/O device specified by $r[B].
/// If no character is available, then $r[C] is set to 0xffffffff;
/// otherwise $r[C] is set to the character read from the I/O device.

pub fn input(machine: &mut UniversalMachine, rc: usize) {
    // Implementation for Input
    // $r[C] := read a character from input, or -1 if no character is available
    let mut buffer = [0; 1];
    let val = match stdin().read(&mut buffer) {
        Ok(0) => 0xffffffff,
        Ok(_) => buffer[0] as u32,
        Err(e) => {
            eprintln!("Error reading from stdin: {}", e);
            0xffffffff
        },
    };

    machine.registers[rc] = val;
}
/// Load a program segment $m[$r[B]] into segment 0.
/// The execution of the current program is terminated.
/// The program counter is set to point to $m[0][$r[C]].
/// If $r[B] = 0, the load-program operation is expected to
/// 
pub fn loadprogram(machine: &mut UniversalMachine, rb: usize, rc: usize) {
    let segment = machine.registers[rb] as usize;
    if segment != 0 {
        let new_segment = machine.memory[segment].clone();
       
        machine.memory[0] = new_segment;
    }
    machine.program_counter = machine.registers[rc] as usize;
}
/// Load a value $r[C] into $r[A].
/// The value $r[C] is loaded, even if it is not a valid
pub fn loadvalue(machine: &mut UniversalMachine, rl: usize, vl: usize) {
   
    machine.registers[rl] = vl as u32;
}
