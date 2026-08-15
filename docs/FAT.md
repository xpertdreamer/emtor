# FAT - F*cked Assembly Translator
> Tiny programm to turn text-assembly into hex programm

The idea of this tool is simpler than ever. It takes a file you give to it and matches each instruction with its hex value to return an array of hex values.
<br>At the first stage, the goal is to **parse->tokenize->translate->return array of bytes**.
<br>For now it doesn`t support labels and some more additional stuff, but it will be implemented later (for now we'll have to be deal with hexadecimal numbers).
<br>This should interact with the emulator's Rust code via FFI. To do this, there will be a function that returns an array of ``uint8_t``, which (in theory, I don't know for sure) should be compatible with ``u8`` from Rust.
