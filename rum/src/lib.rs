pub mod rumdis;
pub mod rumload;
pub mod execution;
pub mod opcodes;



#[cfg(test)]
mod tests { 
use crate::execution::UniversalMachine;
use crate::opcodes::{cmov,load,store,add,mult,div,nand};


#[test]
fn test_cmov() {
    let mut machine = UniversalMachine::new(&Vec::new());
    machine.registers[0] = 1;
    machine.registers[0] = 1;
    machine.registers[1] = 2;
    machine.registers[2] = 3;
    cmov(&mut machine, 0, 1, 2);
    assert_eq!(machine.registers[0], 2);

}

#[test]
fn test_load() {
    let mut machine = UniversalMachine::new(&vec![0,3]);
    machine.registers[0] = 1;
    machine.registers[1] = 0;
    machine.registers[2] = 4;
    load(&mut machine, 2,1, 0);
    assert_eq!(machine.registers[2], 3);

}

#[test]
fn test_store() {
    let mut machine = UniversalMachine::new(&Vec::new());
    machine.registers[0] = 1;
    machine.registers[1] = 2;
    machine.registers[2] = 3;
    store(&mut machine, 0, 1, 2);
    assert_eq!(machine.registers[2], 3);

}

#[test]
fn test_add() {
    let mut machine = UniversalMachine::new(&Vec::new());
    machine.registers[0] = 1;
    machine.registers[1] = 2;
    machine.registers[2] = 3;

    add(&mut machine, 0, 1, 2);
    add(&mut machine, 1, 2, 3);
    assert_eq!(machine.registers[0], 3);
    assert_eq!(machine.registers[1], 5);
}

#[test]
fn test_mult() {
    let mut machine = UniversalMachine::new(&Vec::new());
    machine.registers[0] = 1;
    machine.registers[1] = 2;
    machine.registers[2] = 3;

    mult(&mut machine, 0, 1, 2);
    mult(&mut machine, 1, 2, 3);
    assert_eq!(machine.registers[0], 2);
    assert_eq!(machine.registers[1], 6);


}

#[test]
fn test_div() {
    let mut machine = UniversalMachine::new(&Vec::new());
    machine.registers[0] = 1;
    machine.registers[1] = 2;
    machine.registers[2] = 3;

    div(&mut machine, 0, 1, 2);
    div(&mut machine, 1, 2, 0);
    assert_eq!(machine.registers[0], 0);
    assert_eq!(machine.registers[1], 0);
    assert_eq!(machine.registers[2], 3);
}

#[test]

fn test_nand() {
    let mut machine = UniversalMachine::new(&Vec::new());
    machine.registers[1] = 10; 
    machine.registers[2] = 12; 
    nand(&mut machine, 0, 1, 2);
    assert_eq!(machine.registers[0], !(10 & 12)); // Should be bitwise NAND of 10 and 12

}



}