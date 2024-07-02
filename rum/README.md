# rum-binaries

This is a collection of Universal Machine binaries for the ICFP 2006 programming contest's "Universal Machine".

This Universal Machine was created in the Rust programming language by Nicholas Leffray and Chris Lawler

Help from others: 
Vincent Zhuang
Daniel Diaz
Nick Mendez
Ayman Soudouk

We have created a Rust Universal Machine with the philosophy that we don't care what specific information the programmer passes into the program, we only care about how we can process and store that information. The whole point of a universal machine is to process information stored in programs passed in by users by performing the operations that our universal machine is capable of. This universal machine emulates segmented memory where program instructions are stored and registers where values are to be processed and operated on. 

In our implementation, we have correctly implemented the opcode operations that perform arithmetic on values from registers depending on the instructions stored in segmented memory. In addition, we have also correctly implemented the bitshift operation to extract information stored in a 4-bit opcode from a 32-bit word and to extract information 

In our proposed design, we originally stated that we would use Doctor Daniel's implementation of Bitpack, but we ended up not using it in our final implementation. From our design document, we did not use rumexecution.rs, rummemory.rs, and rumtypes.rs. 

The architecture of our program consists of six modules:
- main.rs
- lib.rs
- execution.rs
- opcodes.rs
- rumdis.rs
- rumload.rs

In the main.rs module, it takes a .um/.umz file as input and stores each instruction in a vector of u32's using the rumload.rs module, which will be discussed later. An instruction set is then defined as an empty vector and our um variable, of type UniversalMachine, is defined. Then, for every instruction we call a function called execute_instruction which is located in the execution.rs module.

The execution.rs module is where the structs that represent the Universal Machine's memory and registers are defined and called. This is also where registers a, b, and c are defined along with the register location and value location variables. Most importantly, this is where the opcode functions are being executed with a match statement. 

The opcodes.rs module is where each opcode is defined. These opcodes are cmov, load, store, add, mult, div, nand, halt, map, unmap, output, input, loadprogram, and loadvalue. These are the operations that the Universal Machine relies on to perform arithmetic on the values stored in registers.

The rumdis.rs module is where the bitshifting operations are located to correctly extract the 4-bit opcode and 3-bit a, b, and c registers stored in each 32-bit instruction. 

The lib.rs module is where each module is linked and made public so resources from each module can be used elsewhere. In addition, this is also where the tests for each opcode are executed. 

On an Intel-based MacBook, it takes the universal machine 27 seconds to execute 2113497560 instructions. On an ARM-based Apple Silicon MacBook, it takes the universal machine 9 seconds to execute 2113497560 instructions. Therefore, using the 9-second resulted test, it would take the universal machine 0.213 seconds to execute 50 million instructions. 

We have spent about five hours analyzing this assignment and ten hours preparing our design. 

We have spent over 15 hours solving this problem after the analysis took place.