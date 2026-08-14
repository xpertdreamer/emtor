// parse -> tokenize -> translate

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

typedef struct {
  const char *opcode;
  const uint8_t hex;
  // NOTE: maybe in will be better to rely on the number of arguments
  const uint8_t argsize; // in bytes
} Op;

Op table[] = {
    {"hlt", 0x00, 0},
    {"add", 0x01, 0},
    {"mov", 0x02, 2},
    {"sub", 0x03, 0},
    {"mul", 0x04, 0},
    {"jmp", 0x05, 2},
    {"cmp", 0x06, 0},
    {"jct", 0x07, 3},
    {"inc", 0x08, 2},
    {"dec", 0x09, 2},
    {"ofs", 0x0A, 2},
    {"jor", 0x0B, 3},
    {"nop", 0x0C, 0},
    {"str", 0x0D, 3},
    {"lmr", 0x0E, 3},
    {"moc", 0x0F, 2},
    {"not", 0x10, 1},
    {"xor", 0x11, 2},
    {"bor", 0x12, 2},
    {"and", 0x13, 2},
    {"jof", 0x14, 2},
    {"psh", 0x15, 1},
    {"pop", 0x16, 1},
    {"mod", 0x17, 0},
    {"iwg", 0x18, 2},
    {"gmb", 0x19, 0},
    {"div", 0x20, 0},
    {"sht", 0x21, 2},
    {"shc", 0x22, 2},
    {"rtr", 0x23, 2},
    {"bsl", 0x24, 2}
};

#define TABLESIZE (sizeof(table) / sizeof(*table))

typedef struct {
    char *data;
    size_t size;
} Buffer;

Buffer read_file(char *filename) {
    char *buf = NULL;
    long len;
    FILE *fptr = fopen(filename, "r");
    if (fptr == NULL) {
        return (Buffer){.data = NULL, .size = 0};
    }
    if (fseek(fptr, 0, SEEK_END) != 0) {
        fclose(fptr);
        return (Buffer){.data = NULL, .size = 0};
    }
    len = ftell(fptr);
    if (len < 0) {
        fclose(fptr);
        return (Buffer){.data = NULL, .size = 0};
    }
    if (len == 0) {
        fclose(fptr);
        return (Buffer){.data = NULL, .size = 0};
    }
    rewind(fptr);
    buf = malloc((size_t)len + 1);
    if (!buf) {
        fclose(fptr);
        return (Buffer){.data = NULL, .size = 0};
    }
    size_t n = fread(buf, 1, (size_t)len, fptr);
    fclose(fptr);
    buf[n] = '\0';
    return (Buffer){.data = buf, .size = n};
}
