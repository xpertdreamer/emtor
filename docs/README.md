> Not fully implemented. Documentation is sucks

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

### HLT command 
Terminates program execution. Doesn`t have any arguments
[back](#opcodes)

### ADD command 
Doesn`t have any arguments. Simply performs the addition of two registers (a + b). The result is stored in register c (0x02).
[back](#opcodes)

### SUB command
Similar to ADD. Produces c = b - a operation
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

## CPU Flags Layout
The CPU uses a single byte flag register to store the results of comparisons and operations. Each bit represents a specific condition of compare. Do not mistake these for system flags (not implemented yet).

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
