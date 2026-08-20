// parse -> tokenize -> translate

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#ifndef _WIN32
#define ERROR_COLOR "\e[1;31m"
#define DEBUG_COLOR "\e[1;34m"
#define RESET_COLOR "\e[0m"
#else
#define ERROR_COLOR ""
#define DEBUG_COLOR ""
#define RESET_COLOR ""
#endif

#define ERROR(fmt, ...)                                                 \
    do {                                                                \
        fprintf(stderr, "%sERROR:%s ", ERROR_COLOR, RESET_COLOR);       \
        fprintf(stderr, fmt, ##__VA_ARGS__);                            \
    } while (0)

#define DEBUG(fmt, ...)                                             \
    do {                                                            \
        fprintf(stdout, "%sDEBUG:%s ", DEBUG_COLOR, RESET_COLOR);   \
        fprintf(stdout, fmt, ##__VA_ARGS__);                        \
    } while (0)

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
  const uint8_t argsize; // in bytes (label name counts as 1 byte)
} Op;

static const Op table[] = {
    {"hlt", 0x00, 0},
    {"add", 0x01, 0},
    {"mov", 0x02, 2},
    {"sub", 0x03, 0},
    {"mul", 0x04, 0},
    {"jmp", 0x05, 1},
    {"cmp", 0x06, 2},
    {"jct", 0x07, 2},
    {"inc", 0x08, 1},
    {"dec", 0x09, 1},
    {"ofs", 0x0A, 2},
    {"jor", 0x0B, 2},
    {"nop", 0x0C, 0},
    {"str", 0x0D, 3},
    {"lmr", 0x0E, 3},
    {"moc", 0x0F, 2},
    {"not", 0x10, 1},
    {"xor", 0x11, 2},
    {"bor", 0x12, 2},
    {"and", 0x13, 2},
    {"jof", 0x14, 1},
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

#define TABLESIZE                    (sizeof(table) / sizeof(*table))
#define INITIAL_LABEL_TABLE_CAPACITY 10

#define DELIMITER    " "
#define LABEL_MARK   "#"
#define COMMENT_MARK ';'

#define BASE_16 16
#define BASE_10 10
#define BASE_2  2

#define BASE_PREFIX_SIZE 2

#define BASE_16_PREFIX   "0x"
#define BASE_2_PREFIX    "0b"

#define MAX_SIZE_IN_BYTES 112

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
        ERROR("Error occured while parsing number. Returning 0");
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

void strip_comments(Buffer *buf) {
    if (buf == NULL || buf->data == NULL) return;
    char* b = buf->data;
    char in = 0;
    while (*b != '\0') {
        if ((int)*b == COMMENT_MARK) {
            *b = ' ';
            in = 1;
        }
        if (*b == '\n') in = 0;
        if (in) *b = ' ';
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

LabT init_lable_table() {
    Label* buck = malloc(sizeof(*buck) * INITIAL_LABEL_TABLE_CAPACITY);
    if (buck == NULL) {
        ERROR("Can not initialize table of labels (buck is NULL)\n");
        return (LabT){.bucket = NULL, .cap = 0, .size = 0};
    }
    return (LabT){.bucket = buck, .cap = INITIAL_LABEL_TABLE_CAPACITY, .size = 0};
}

uint16_t find_label(LabT* table, const char* name) {
    if (table == NULL || table->bucket == NULL) {
        ERROR("Failed to find label '%s'\n", name);
        return 0;
    }
    for (size_t i = 0; i < table->size; ++i) {
        if (strcmp(table->bucket[i].name, name) == 0)
            return table->bucket[i].address;
    }
    ERROR("Failed to translate, label '%s' not found in table\n", name);
    return 0;
}

char label_exists(LabT* table, const char* name) {
    if (table == NULL || table->bucket == NULL) return 0;
    for (size_t i = 0; i < table->size; ++i) {
        if (strcmp(table->bucket[i].name, name) == 0)
            return 1;
    }
    return 0;
}

char add_lable_table(LabT* table, Label* label) {
    if (table == NULL || table->bucket == NULL || label == NULL) {
        ERROR("Failed to add label to table\n");
        return 0;
    }
    if (label_exists(table, label->name)) {
        ERROR("Duplicate label %s\n", label->name);
        return 0;
    }
    // check if table capacity colliding with size and resize if needed
    if (table->cap == table->size) {
        size_t cap_rep = table->cap == 0 ? INITIAL_LABEL_TABLE_CAPACITY : table->cap * 2;
        Label* bucket_rep = realloc(table->bucket, sizeof(*bucket_rep) * cap_rep);
        if (bucket_rep == NULL) {
            ERROR("Memory reallocation failed in add_label");
            return 0;
        }
        table->bucket = bucket_rep;
        table->cap = cap_rep;
    }
    table->bucket[table->size] = *label;
    table->size++;
    return 1;
}

char collect_labels(Tokenized* buf, LabT* label_table) {
    if (buf == NULL || buf->buf == NULL || buf->size == 0) return 0;
    size_t c = 0;
    for (size_t i = 0; i < buf->size; i++) {
        const char* token = buf->buf[i];
        char found = 0;
        if (strcmp(token, LABEL_MARK) == 0) {
            if (i + 1 >= buf->size) {
                ERROR("Label mark with no label name at address %zu\n", c);
                return 0;
            }
            ++i;
            add_lable_table(label_table, &(Label){.name = strdup(buf->buf[i]), .address = c});
            DEBUG("New label at address %zu\n", c);
            continue;
        }
        for (int j = 0; j < (int)TABLESIZE; ++j) {
            if (strcmp(token, table[j].opcode) == 0) {
                c++;
                found = 1;
                if (table[j].argsize > 0) {
                    for (int k = 0; k < table[j].argsize; ++k) {
                        ++i;
                        if (i >= buf->size) {
                            ERROR("Memory out of bound during collecting labels at byte %#x\n", (unsigned int)i);
                            return 0;
                        }
                        if (strcmp(token, "jmp") == 0 || strcmp(token, "jof") == 0) {
                            c+=2;
                            continue;
                        }
                        if ((strcmp(token, "jor") == 0 || strcmp(token, "jct" ) == 0) && k == 1) {
                            c+=2;
                            continue;
                        }
                        c++;
                    }
                }
                break;
            }
        }
        if (found == 0) {
            ERROR("Unknown instruction occoured at byte %#x. Abort\n", (unsigned int)i);
            return 0;
        }
    }
    return 1;
}

uint8_t* translate(Tokenized* buf, size_t* out, LabT* label_table) {
    // TODO: data block
    if (buf == NULL || buf->buf == NULL || buf->size == 0) return NULL;
    uint8_t* arr = malloc(buf->size*sizeof(uint8_t));
    if (!arr) return NULL;
    size_t c = 0;
    for (size_t i = 0; i < buf->size; i++){
        const char* token = buf->buf[i];
        if (strcmp(token, LABEL_MARK) == 0) {
            i ++;
            continue;
        }
        for (int j = 0; j < (int)TABLESIZE; ++j) {
            if (strcmp(token, table[j].opcode) == 0) {
                arr[c++] = table[j].hex;
                if (table[j].argsize > 0) {
                    for (int k = 0; k < table[j].argsize; ++k) {
                        ++i;
                        const char* arg = buf->buf[i];
                        if ((strcmp(token, "jmp") == 0 || strcmp(token, "jof") == 0)
                            || ((strcmp(token, "jor") == 0 || strcmp(token, "jct" ) == 0) && k == 1)) {
                            uint16_t label_addr = find_label(label_table, arg);
                            uint8_t high = (uint8_t)(label_addr >> 8);
                            uint8_t low = (uint8_t)(label_addr & 0xFF);
                            DEBUG("Jump to %u %u\n", high, low);
                            arr[c++] = high;
                            arr[c++] = low;
                            continue;
                        }
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
    }
    uint8_t* res = realloc(arr, c * sizeof(uint8_t));
    *out = c;
    return res ? res : arr;
}

uint8_t* fat(const char* filename, size_t* size) {
    Buffer buf = read_file(filename);
    if (buf.data == NULL) {
        ERROR("Failed to read file %s\n", filename);
        return NULL;
    }
    strip_comments(&buf);
    strip_nl(&buf);
    Tokenized res = tokenize(&buf);
    if (res.buf == NULL) {
        ERROR("Tokenization failed\n");
        free(buf.data);
        return NULL;
    }
    LabT label_table = init_lable_table();
    if (collect_labels(&res, &label_table) == 0) {
        ERROR("Label collection failed\n");
        free(buf.data);
        free(res.buf);
        for (size_t i = 0; i < label_table.size; ++i) free((void*)label_table.bucket[i].name);
        free(label_table.bucket);
        return NULL;
    }
    size_t s = 0;
    uint8_t* translated = translate(&res, &s, &label_table);
    free(buf.data);
    free(res.buf);
    for (size_t i = 0; i < label_table.size; ++i) free((void*)label_table.bucket[i].name);
    free(label_table.bucket);
    if (s > MAX_SIZE_IN_BYTES) {
        ERROR("Code is bigger than memory size\n");
        return NULL;
    }
    if (translated == NULL) {
        ERROR("Some arguments not providen, or error\n");
        return NULL;
    }
    if (size != NULL) *size = s;
    fflush(stdout);
    fflush(stderr);
    return translated;
}

void free_translated(uint8_t* ptr) {
    if (ptr != NULL) {
        free(ptr);
    }
}
