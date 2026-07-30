# emtor
> Recreational project which have the goal - write an emulator of fictional architecture in Rust

### OPCODES
These are test opcodes and maybe will be changed later

| OPCODE | Command           |
|--------|-------------------|
| 0x00   | HLT               |
| 0x01   | ADD               |
| 0x02   | MOV mode dest src |
| 0x03   | SUB               |
| 0x04   | MUL               |
| 0x05   | JMP address       |
| 0x06   | CMP               |
| 0x07   | JCT mask address  |

### HLT command
Terminates program execution. Doesn`t have any arguments

### ADD command
Doesn`t have any arguments. Simply performs the addition of two registers. The result is stored in register a (0x00).

### SUB command
Similar to ADD. Produces a=a-b operation

### MOV command
Mode argument and two operands lay down in memory sequentially

| CODE | MOV OPERATING MODE                       |
|------|------------------------------------------|
| 0x0A | From register to register (a = 0, b = 1) |
| 0x0B | From memory to register (src = value)    |

### MUL command
Similar to ADD. Produces a=a*b operation

### JMP command
Accept the next address of Programm Counter (PC). The address consists of two operands (high and low bytes), which are stored in memory sequentially in Big-Endian order: 

``[JMP Instruction -> High byte -> Low byte]``

and then they form a single address: 

``[HIGH|LOW]``

### CMP command
Compares register A with register B and sets the flag register accordingly. All flags are updated based on the comparison result. The flag layout is given in the section [CPU Flags Layout](#cpu-flags-layout)

### JCT command
Performs a conditional jump based on the flag mask. The instruction checks if all bits in the mask are set in the flag register. Format: JCT mask, address

### CPU Flags Layout
The CPU uses a single byte flag register to store the results of comparisons and operations. Each bit represents a specific condition.

| Bit | Flag | Name |
|-----|------|------|
| 7 | `NZ` | Not Zero |
| 6 | `ZE` | Zero |
| 5 | `LE` | Less or Equal |
| 4 | `GE` | Greater or Equal | 
| 3 | `LT` | Less Than |
| 2 | `GT` | Greater Than |
| 1 | `NE` | Not Equal |
| 0 | `EQ` | Equal |
