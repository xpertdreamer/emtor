# FAT - F*cked Assembly Translator
> Tiny programm to turn text-assembly into hex programm

> [!NOTE]
> Source code can be messy and unoptimized because I don`t give a shit about it at this stage.

The idea of this tool is simpler than ever. It takes a file you give to it and matches each instruction with its hex value to return an array of hex values.
<br>At the first stage, the goal is to **parse->tokenize->translate->return array of bytes**.
<br>Since *[17/08/2026]* directly supports register names & different code bases in code and since *[19/08/2026]* fully supports labels and comments.
<br>This library interact with the emulator's Rust code via FFI. To do this, there are a function that returns an array of ``uint8_t``, which compatible with ``u8`` from Rust.
<br><br>Syntax is pretty close to x86 assembly. The only difference is the absence of commas when listing arguments. For example:

``` asm
moc A 1
mov B A
add
and A C
cmp A C
jor 0x02 label
hlt
# label
inc C
```

The translator is written in C and is intended to be used as a backend for the emulator. The C library compiles during the project building process and links with the Rust emulator code. This interact with the emulator's Rust code via FFI (Foreign Function Interface). The interaction is handled on the Rust side

The C code exposes two functions to Rust:

- ``fat()`` – takes a filename and a pointer to a size variable, returns a pointer to the translated byte array.
- ``free_translated()`` - frees the allocated memory after the Rust code has finished using it.

<br>Below is an example of the FFI interaction in Rust:

```rust
unsafe extern "C" {
    fn fat(filename: *const c_char, size: *mut usize) -> *mut u8;
    fn free_translated(ptr: *mut u8);
}

pub fn load_rom(&mut self, filename: String) {
    unsafe {
        let c_filename = CString::new(filename).expect("New CString failed");
        let mut len: usize = 0;
        let ptr = fat(c_filename.as_ptr(), &mut len);
        if !ptr.is_null() && len > 0 {
            self.load_program(slice::from_raw_parts(ptr, len));
            free_translated(ptr);
        } else if !ptr.is_null() {
            self.trace("Size returned by C code is 0");
            free_translated(ptr);
        }
    }
    self.trace("Rom loaded");
}
```

In this Rust code, the load_rom method does the following:

    1. Converts the Rust string into a C-compatible string.

    2. Calls fat() to translate the assembly file into a byte array.

    3. Checks the returned pointer and size:

       * If valid, it loads the program into the emulator using load_program() and then frees the memory.

       * If the size is zero, it logs a warning and still frees the memory.

    4. Logs a confirmation message once the ROM is loaded.

