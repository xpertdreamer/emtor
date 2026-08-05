> Not fully implemented. Documentation is sucks

### TODO: CALL, RET instructions 
### TODO?: stack
### TODO: PUSH, POP instructions should interact with memory
### TODO: 3 sections memory layout
### TODO: system flags (sign handler, overflow handler and etc.)
### TODO: tests
### TODO: error handling

# emtor
> Recreational project which have the goal to write an emulator of fictional architecture in Rust

## OPCODES

| OPCODE | Command                          |
|--------|----------------------------------|
| 0x00   | [HLT](#hlt-command)              |
| 0x01   | [ADD](#add-command)              |
| 0x02   | [MOV dest src](#mov-command)     |
| 0x03   | [SUB](#sub-command)              |
| 0x04   | [MUL](#mul-command)              |
| 0x05   | [JMP address](#jmp-command)      |
| 0x06   | [CMP](#cmp-command)              |
| 0x07   | [JCT mask address](#jct-command) |
| 0x08   | [INC address](#inc-command)      |
| 0x09   | [DEC address](#dec-command)      |
| 0x0A   | [OFS offset](#ofs-command)       |
| 0x0B   | [JOR mask address](#jor-command) |
| 0x0C   | [NOP](#nop-command)              |
| 0x0D   | [STR reg address](#str-command)  |
| 0x0E   | [LMR reg address](#lmr-command)  |
| 0x0F   | [MOC dest const](#moc-command)   |
| 0x10   | [NOT reg](#not-command)          |
| 0x11   | [XOR dest src](#xor-command)     |
| 0x12   | [BOR dest src](#bor-command)     |
| 0x13   | [AND dest src](#and-command)     |
| 0x14   | [JOF address](#jof-command)      |
| 0x15   | [PSH src](#psh-command)          |
| 0x16   | [POP dest](#pop-command)         |


### HLT command 
Terminates program execution. Doesn`t have any arguments
[back](#opcodes)

### ADD command 
Doesn`t have any arguments. Simply performs the addition of two registers (a + b). The result is stored in register c (0x02). Cause the calculation of OF, SF, CF.
[back](#opcodes)

### SUB command
Similar to ADD. Produces c = b - a operation. Cause the calculation of OF, SF, CF.
[back](#opcodes)

### MOV command 
Two operands lay down in memory sequentially. Copies the value from register 'dest' to register 'src'.
[back](#opcodes)

### MOC command  
Similar to MOV, but accepts a constant value as a second argument. Copies this constant to register 'dest'.
[back](#opcodes)

### MUL command 
Similar to ADD. Produces c = a * b operation
[back](#opcodes)

### JMP command 
Accept the next address of Programm Counter (PC). The address consists of two operands (high and low bytes), which are stored in memory sequentially in Big-Endian order: 

``[JMP Instruction -> High byte -> Low byte]``

and then they form a single address: 

``[ HIGH | LOW ]``

[back](#opcodes)

### CMP command 
Compares register A with register B and sets the flag register accordingly. All flags are updated based on the comparison result. The flag layout is given in the section [CPU Flags Layout](#cpu-flags-layout)
[back](#opcodes)

### JCT command 
Performs a conditional jump based on the flag mask. The instruction checks if all bits in the mask are set in the flag register. Format: JCT mask address
[back](#opcodes)

### JOR command 
Performs a conditional jump based on the flag mask. The instruction checks if a bit in the mask are set in the flag register. Format: JOR mask address.
If mask given as an argument contains more than one 'true' bit - throws error.
[back](#opcodes)

### JOF command 
Performs a conditional jump based on the overflow flag (OF) status. The instruction checks if OF (0x02) bit is set in the flag register. Format: JOF address
[back](#opcodes)

### INC command 
Increments a number in a given register
[back](#opcodes)

### DEC command 
Decrements a number in a given register
[back](#opcodes)

### OFS command 
The full name is OFFSET. Moves the PC by the number passed as an argument. The address consists of two operands (high and low bytes), which are stored in memory sequentially in Big-Endian order ([Check this](#jmp-command))
[back](#opcodes)

### NOP command 
Little fattie. Takes up space in memory
[back](#opcodes)

### STR command 
The full name is STORE. Just copies the value from register to memory.
[back](#opcodes)

### LMR command 
The full name is Load from Memory to Register. Copies the value from the memory location represented by the address into a register
[back](#opcodes)

### NOT command 
Bitwise complement of the given register`s value
[back](#opcodes)

### XOR command 
Performs XOR operation on destination by source. The result is stored in 'destination'.
[back](#opcodes)

### BOR command 
Performs Bitwise OR operation on destination by source. The result is stored in 'destination'.
[back](#opcodes)

### AND command 
Performs AND operation on destination by source. The result is stored in 'destination'.
[back](#opcodes)

### PSH command
The full name is PUSH. Temporarily accepts only register address to copy (push) value from it to stack. You should know how stack is working.
<br> May be used in situations when value from specific register need to be stored in safe place. <br>
[back](#opcodes)

### POP command
Another one from stack section. Extracts the value on top of the stack into given register. <br>
[back](#opcodes)

## CPU Flags Layout

### Compare flags
The CPU uses a single byte flag register to store the results of comparisons. Each bit represents a specific condition of compare. Do not mistake these for system flags.

| Bit | Flag | Name             |
|-----|------|------------------|
| 7   | `NZ` | Not Zero         |
| 6   | `ZE` | Zero             |
| 5   | `LE` | Less or Equal    |
| 4   | `GE` | Greater or Equal |
| 3   | `LT` | Less Than        |
| 2   | `GT` | Greater Than     |
| 1   | `NE` | Not Equal        |
| 0   | `EQ` | Equal            |

### System flags (not propertly implemented yet)
The CPU uses a single byte flag register to store the results of operations. Each bit represents a specific condition of operation result.

| Bit | Flag | Name          |
|-----|------|---------------|
| 2   | `SF` | Sign Flag     |
| 1   | `OF` | Overflow flag |
| 0   | `CF` | Carry Flag    |

## Memory layout
Currently, the RAM size is 256 bytes, 16 of which are occupied by the stack. A schematic representation is shown below (will be replaced with 3 section memory):

`` [   Free: 240 bytes   |   Stack: 16 bytes   ] ``

As illustrated, the stack resides within the  upper portion of memory, leaving 240 bytes for other data, variables, and program use.
It should be noted that this data is relevant at the current stage of development [02.08.2026] and the memory is likely to be expanded in the future.
As usual, the CPU has a PC (Program Counter) and an SP (Stack Pointer). The first one holds the address of the next instruction in memory to be executed, and 
the second one holds the address of the current stack top...
