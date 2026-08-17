> Not fully implemented. Documentation is sucks

### TODO: dynamic memory size
### TODO: PUSH, POP instructions should interact with memory
### TODO: timer and interaprions
### TODO: io
### TODO: system flags (sign handler, overflow handler and etc.)
### TODO: tests with load_rom
### TODO: create examples
### TODO: tests
### TODO: error handling
### IN PLAN: 4 sections memory layout (Data Pointer)
### IN PLAN: LEA
### IN PLAN: syscalls
### IN PLAN: parser & other cool stuff to load programs from file

# emtor
> Recreational project which have the goal to write an emulator of fictional architecture in Rust

## Table of Contents

- [Build and Run](#build-and-run)
  - [ROM Loading](#rom-loading)
  - [Running Examples](#running-examples)
- [Opcodes](#opcodes)
- [CPU Flags Layout](#cpu-flags-layout)
  - [Compare Flags](#compare-flags)
  - [System Flags](#system-flags-not-propertly-implemented-yet)
- [Memory Layout](#memory-layout)
- [TODO](#todo)
- [IN PLAN](#in-plan)
- [FAT documentation](FAT.md)


## Build and run
To build **emtor** run:

``` bash
make 
```

To build and run tests:

``` bash
make test
``` 

To build and run only specific tests:

``` bash
make test <output=1/0> ARGS="<KEYWORD>"
``` 

To run static analysis (linter):

``` bash
make clippy
```

To clean all build artifacts:

``` bash
make clean
```

### ROM loading

To load an assembly file into emtor and execute it you should provide a path to `.emt` (recommended, can be any file format) file as an argument:

``` bash
cd target && ./emtor <path-to-file>
```

For now there are no validation during translation, so please be careful.

### Running examples

> [!WARNING]
> Before running any script, please check its code

You can also run examples located in the corresponding directory with providen script:

``` bash
sh run_example.sh
```

or 

``` bash
chmod +x run_example.sh
./run_example.sh
```

## OPCODES

| OPCODE | Command                          |
|--------|----------------------------------|
| 0x00   | [HLT](#hlt-command)              |
| 0x01   | [ADD](#add-command)              |
| 0x02   | [MOV dest src](#mov-command)     |
| 0x03   | [SUB](#sub-command)              |
| 0x04   | [MUL](#mul-command)              |
| 0x05   | [JMP address](#jmp-command)      |
| 0x06   | [CMP reg reg](#cmp-command)      |
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
| 0x17   | [MOD](#mod-command)              |
| 0x18   | [IWG address](#iwg-command)      |
| 0x19   | [GMB](#gmb-command)              |
| 0x20   | [DIV](#div-command)              |
| 0x21   | [SHT data dest](#sht-command)    |
| 0x22   | [SHC data dest](#shc-command)    |
| 0x23   | [RTR data dest](#rtr-command)    |
| 0x24   | [BSL data dest](#bsl-command)    |


### HLT command 
Terminates program execution. Doesn`t have any arguments
[back](#opcodes)

### ADD command 
Doesn`t have any arguments. Simply performs the addition of two registers (a + b). The result is stored in register c (0x02). Cause the calculation of OF, SF, CF.
[back](#opcodes)

### SUB command
Similar to ADD. Produces c = b - a operation. Cause the calculation of OF, SF, CF.
[back](#opcodes)

### MOD command
Similar to ADD. Produces c = a % b operation.
[back](#opcodes)

### DIV command
Similar to ADD. Produces c = b / a operation. If division by zero - sets OF to true and write 0 as result to register C. 
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
Compares register ``left`` with register ``right`` and sets the flag register accordingly. All flags are updated based on the comparison result. The flag layout is given in the section [CPU Flags Layout](#cpu-flags-layout)
<br>[back](#opcodes)

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
The full name is STORE. Just copies the value from register to memory. The destination address is specified relative to the start of the data segment. The actual physical address is calculated as ``DATA_SEG_START + offset``, where ``DATA_SEG_START`` is the beginning of the data memory section. For example, if ``DATA_SEG_START = 0x0070`` and you write STR A 0x00 0x05, the value from register A will be stored at physical address 0x0075.
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

### SHT command
The full name is Shift That. Accepts two operands: 
<br>1. Data – contains the shifting direction in the high bit (similar to a negative number), where 0 means left shift and 1 means right shift, along with the number of shift operations.
<br>2. Destination – the destination register will be overwritten with the result of the operation.
<br>After execution, the command restores the high bit.
<br>[back](#opcodes)

### SHC command
The full name is Shift with Carry. Accepts two operands: 
<br>1. Data – contains the shifting direction in the high bit (similar to a negative number), where 0 means left shift and 1 means right shift, along with the number of shift operations.
<br>2. Destination – the destination register will be overwritten with the result of the operation.
<br>After execution, the command substitutes the value of the carry flag into the vacated extreme bit position.
<br>[back](#opcodes)

### RTR command
The full name is RoTate, Retard! Accepts two operands:
<br>1. Data – contains the shifting direction in the high bit (similar to a negative number), where 0 means left shift and 1 means right shift, along with the number of shift operations.
<br>2. Destination – the destination register will be overwritten with the result of the operation.
<br>During execution bits falling off one end of a binary number wrap around to the other end.
<br>[back](#opcodes)

### BSL command
The full name is Bit Shift Logic. 
The full name is RoTate, Retard! Accepts two operands:
<br>1. Data – contains the shifting direction in the high bit (similar to a negative number), where 0 means left shift and 1 means right shift, along with the number of shift operations.
<br>2. Destination – the destination register will be overwritten with the result of the operation.
<br>During execution bits fallig off one end of a binary number replace with zeros.

### PSH command
The full name is PUSH. Temporarily accepts only register address to copy (push) value from it to stack. You should know how stack is working.
<br> May be used in situations when value from specific register need to be stored in safe place. <br>
NOTE: Here stack growth ascending (sp += 1).
[back](#opcodes)

### POP command
Another one from stack section. Extracts the value on top of the stack into given register. <br>
[back](#opcodes)

### IWG command
The full name is I Wanna Go. Analogue of CALL from x86 asm. Accepts address as two bytes value and jump on it. Pre-saving the current address before the jump onto the call stack for later return. An address is stored onto the call stack as follows:

``
CALL STACK TOP -> [ HIGH BYTE | LOW BYTE  | ...  ] 
``

NOTE: Here call stack growth ascending (csp += 1), as does the data stack.
[back](#opcodes)

### GMB command
The full name is Give Me Back. It is an analogue of RET from x86 assembly. Currently, it doesn't really do anything except extract the return address from the call stack and jump to it.
<br>NOTE: I want to implement return values with this instruction.
<br>[back](#opcodes)

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
Currently, the RAM size is 256 bytes, 16 of which are occupied by the stack and other 16 by call stack. A schematic representation is shown below:

`` [   Programs: 112 bytes   |     Data: 112 bytes    | Call Stack: 16 bytes |   Stack: 16 bytes   ] ``

As illustrated, the stack resides within the  upper portion of memory, leaving 224 bytes for other data, variables, and program use.

> [!NOTE]
> Data and programs are stored in separate segments. The code segment is read-only and contains the program instructions, while the data segment is read-write and holds other modifiable data (variables and etc.). Memory operations (`STR`, `LMR`) work exclusively with the data segment. The instruction pointer (PC) operates within the code segment.

It should be noted that this data is relevant at the current stage of development [02.08.2026] and the memory is likely to be expanded in the future.
As usual, the CPU has a PC (Program Counter) and an SP (Stack Pointer). The first one holds the address of the next instruction in memory to be executed, and 
the second one holds the address of the current stack top. Additionally, there is a CSP (Call Stack Pointer), which points to the top of the call stack. This pointer is automatically managed by the CPU when IWG and GMB instructions are executed, ensuring that return addresses are properly pushed and popped.
<br>Keep in mind that registers, even though memory is unsigned, are signed and can only store values ​​in the range -128 to 127, while memory ranges from 0 to 256.
