// parse -> tokenize -> translate

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
    const char* name;
    uint16_t address;
} Label;

typedef struct {
    Label* bucket;
    size_t size;
    size_t cap;
} LabT;

typedef struct {
  const char *opcode;
  const uint8_t hex;
  // NOTE: maybe in will be better to rely on the number of arguments
  const uint8_t argsize; // in bytes
} Op;

static const Op table[] = {
    {"hlt", 0x00, 0},
    {"add", 0x01, 0},
    {"mov", 0x02, 2},
    {"sub", 0x03, 0},
    {"mul", 0x04, 0},
    {"jmp", 0x05, 2},
    {"cmp", 0x06, 2},
    {"jct", 0x07, 3},
    {"inc", 0x08, 1},
    {"dec", 0x09, 1},
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
#define INITIAL_LABEL_TABLE_CAPACITY 10

#define DELIMITER " "

#define BASE_16 16
#define BASE_10 10
#define BASE_2  2

#define BASE_PREFIX_SIZE 2
#define BASE_16_PREFIX   "0x"
#define BASE_2_PREFIX    "0b"

LabT init_lable_table() {
    Label* buck = malloc(sizeof(*buck) * INITIAL_LABEL_TABLE_CAPACITY);
    if (buck == NULL) {
        printf("ERROR: Can not initialize table of labels (buck is NULL)\n");
        return (LabT){.bucket = NULL, .cap = 0, .size = 0};
    }
    return (LabT){.bucket = buck, .cap = INITIAL_LABEL_TABLE_CAPACITY, .size = 0};
}

char add_lable_table(LabT* table, Label* label) {
    if (table == NULL || label == NULL) {
        printf("ERROR: Failed to add label to table\n");
        return 0;
    }
    // check if table capacity colliding with size and resize if needed

    return 1;
}

enum register_hash {
   HASH_A = 1154,
   HASH_B = 1155,
   HASH_C = 1156
};

enum register_address {
   REG_A_ADDRESS = 0xC0,
   REG_B_ADDRESS = 0xC1,
   REG_C_ADDRESS = 0xC2,
};

// Reference: http://www.cse.yorku.ca/~oz/hash.html
unsigned long hash(const char* string) {
    unsigned long hash = 33;
    int c;
    while ( ( c = *string++ ) ) {
        hash = ((hash << 5) + hash) + c;
    }
    return hash;
}

uint8_t reg_to_hex(const char* regname) {
    switch (hash(regname)) {
        case HASH_A: return REG_A_ADDRESS;
        case HASH_B: return REG_B_ADDRESS;
        case HASH_C: return REG_C_ADDRESS;
        default:     return 0;
    }
}

uint8_t parse_num(const char* token) {
    if (token == NULL) {
        printf("Error occured while parsing number. Returning 0");
        return 0;
    }
    if (strncmp(token, BASE_16_PREFIX, BASE_PREFIX_SIZE) == 0) {
        return (uint8_t)strtoul(token + BASE_PREFIX_SIZE, NULL, BASE_16);
    }
    if (strncmp(token, BASE_2_PREFIX, BASE_PREFIX_SIZE) == 0) {
        return (uint8_t)strtoul(token + BASE_PREFIX_SIZE, NULL, BASE_2);
    }

    return (uint8_t)strtoul(token, NULL, BASE_10);
}

typedef struct {
    char *data;
    size_t size;
} Buffer;

Buffer read_file(const char *filename) {
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
    if (len <= 0) {
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

void strip_nl(Buffer *buf) {
    if (buf == NULL || buf->data == NULL) return;
    char* b = buf->data;
    while (*b != '\0') {
        if (*b == '\n') *b = ' ';
        b++;
    }
}

typedef struct {
    char** buf;
    size_t size;
} Tokenized;

Tokenized tokenize(Buffer *buf) {
    if (buf == NULL || buf->data == NULL || buf->size == 0) return (Tokenized){.buf = NULL, .size = 0};
    char* copy = strdup(buf->data);
    if (!copy) return (Tokenized){.buf = NULL, .size = 0};
    int c = 0;
    char *p = strtok(copy, DELIMITER);
    while (p != NULL) {
        c++;
        p = strtok(NULL, DELIMITER);
    }
    if (c == 0) {
        free(copy);
        return (Tokenized){.buf = NULL, .size = 0};
    }
    free(copy);

    char** array = malloc(c*sizeof(char*));
    if (!array) return (Tokenized){.buf = NULL, .size = 0};
    int i = 0;
    p = strtok(buf->data, DELIMITER);
    while (p != NULL) {
        array[i++] = p;
        p = strtok(NULL, DELIMITER);
    }

    return (Tokenized){.buf = array, .size = i};
}

uint8_t* translate(Tokenized* buf, size_t* out, LabT* label_table) {
    // TODO: handle labels
    if (buf == NULL || buf->buf == NULL || buf->size == 0) return NULL;
    uint8_t* arr = malloc(buf->size*sizeof(uint8_t));
    size_t c = 0;
    if (!arr) return NULL;
    for (size_t i = 0; i < buf->size; i++){
        const char* token = buf->buf[i];
        char found = 0;
        for (int j = 0; j < (int)TABLESIZE; ++j) {
            if (strcmp(token, table[j].opcode) == 0) {
                arr[c++] = table[j].hex;
                found = 1;
                if (table[j].argsize > 0) {
                    for (int k = 0; k < table[j].argsize; ++k) {
                        ++i;
                        if (i >= buf->size) {
                            printf("Memory out of bound during translation at byte %#x\n", (unsigned int)i);
                            free(arr);
                            *out = 0;
                            return NULL;
                        }
                        const char* arg = buf->buf[i];
                        const uint8_t reg_hex = reg_to_hex(arg);
                        if (reg_hex != 0) {
                            arr[c++] = reg_hex;
                            continue;
                        }
                        uint8_t value = parse_num(arg);
                        arr[c++] = value;
                    }
                }
                break;
            }
        }
        if (found == 0) {
            printf("Unknown instruction occoured at byte %#x. Abort\n", (unsigned int)i);
            free(arr);
            *out = 0;
            return NULL;
        }
    }
    uint8_t* res = realloc(arr, c * sizeof(uint8_t));
    *out = c;
    return res ? res : arr;
}

uint8_t* fat(const char* filename, size_t* size) {
    Buffer buf = read_file(filename);
    if (buf.data == NULL) {
        printf("Error: Failed to read file %s\n", filename);
        return NULL;
    }
    strip_nl(&buf);
    Tokenized res = tokenize(&buf);
    if (res.buf == NULL) {
        printf("Error: Tokenization failed\n");
        free(buf.data);
        return NULL;
    }
    LabT label_table = init_lable_table();
    size_t s = 0;
    uint8_t* translated = translate(&res, &s, &label_table);
    free(buf.data);
    free(res.buf);
    free(label_table.bucket);
    if (translated == NULL) {
        printf("Some arguments not providen, or error\n");
        return NULL;
    }
    if (size != NULL) *size = s;
    return translated;
}

void free_translated(uint8_t* ptr) {
    if (ptr != NULL) {
        free(ptr);
    }
}
