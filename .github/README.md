These are test opcodes and will be changed later

| OPCODE | Command           |
|--------|-------------------|
| 0x00   | HLT               |
| 0x01   | ADD               |
| 0x02   | MOV mode dest src |
| 0x03   | SUB               |

## MOV command
Mode argument and two operands lay down in memory sequentially

| CODE | MOV OPERATING MODE                       |
|------|------------------------------------------|
| 0x0A | From register to register (a = 0, b = 1) |
| 0x0B | From memory to register (src = value)    |
