# emtor
> Recreational project which have the goal - write an emulator of fictional architecture in Rust

## OPCODES

| OPCODE | Command                           |
|--------|-----------------------------------|
| 0x00   | [HLT](#hlt-command)               |
| 0x01   | [ADD](#add-command)               |
| 0x02   | [MOV mode dest src](#mov-command) |
| 0x03   | [SUB](#sub-command)               |
| 0x04   | [MUL](#mul-command)               |
| 0x05   | [JMP address](#jmp-command)       |
| 0x06   | [CMP](#cmp-command)               |
| 0x07   | [JCT mask address](#jct-command)  |
| 0x08   | [INC address](#inc-command)       |
| 0x09   | [DEC address](#dec-command)       |
| 0x0A   | [OFS offset](#ofs-command)        |
| 0x0B   | [JOR mask address](#jor-command)  |

### HLT command
Terminates program execution. Doesn`t have any arguments

### ADD command
Doesn`t have any arguments. Simply performs the addition of two registers. The result is stored in register c (0x02).

### SUB command
Similar to ADD. Produces c=a-b operation

### MOV command
Mode argument and two operands lay down in memory sequentially

| CODE | MOV OPERATING MODE                    |
|------|---------------------------------------|
| 0x0A | From register to register             |
| 0x0B | From memory to register (src = value) |

### MUL command
Similar to ADD. Produces c=a*b operation

### JMP command
Accept the next address of Programm Counter (PC). The address consists of two operands (high and low bytes), which are stored in memory sequentially in Big-Endian order: 

``[JMP Instruction -> High byte -> Low byte]``

and then they form a single address: 

``[HIGH|LOW]``

### CMP command
Compares register A with register B and sets the flag register accordingly. All flags are updated based on the comparison result. The flag layout is given in the section [CPU Flags Layout](#cpu-flags-layout)

### JCT command
Performs a conditional jump based on the flag mask. The instruction checks if all bits in the mask are set in the flag register. Format: JCT mask address

### JOR command
Performs a conditional jump based on the flag mask. The instruction checks if a bit in the mask are set in the flag register. Format: JOR mask address.
If mask given as an argument contains more than one 'true' bit - throws error.

### INC command
Increments a number in a given register

### DEC command
Decrements a number in a given register

### OFS command
The full name is OFFSET. Moves the PC by the number passed as an argument. The address consists of two operands (high and low bytes), which are stored in memory sequentially in Big-Endian order ([Check this](#jmp-command))

## CPU Flags Layout
The CPU uses a single byte flag register to store the results of comparisons and operations. Each bit represents a specific condition.

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
