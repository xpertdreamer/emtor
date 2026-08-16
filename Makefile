NAME=emtor
RUSTC=rustc
RUSTFLAGS=-A unused -O -C link-arg=$(FAT)
CLIPPYFLAGS=-A unused -C link-arg=$(FAT)
CLIPPY=clippy-driver
SRC=src
BUILDDIR=target
TARGET=$(BUILDDIR)/$(NAME)
TEST_TARGET=$(BUILDDIR)/$(NAME)_test
SRCS=$(shell find $(SRC) -name '*.rs')
FAT=fat/libfat.a
FAT_OBJ=fat/fat.o

ifdef output
	override ARGS += --nocapture
endif

.PHONY: all clean test clippy

all: $(TARGET)

$(FAT): fat/fat.c
	gcc -c fat/fat.c -o $(FAT_OBJ)
	ar rcs $(FAT) $(FAT_OBJ)


$(TARGET): $(SRCS) $(FAT)
		   @mkdir -p $(BUILDDIR)
		   $(RUSTC) $(RUSTFLAGS) $(SRC)/main.rs -o $(TARGET)

test: $(SRCS) $(FAT)
	  @mkdir -p $(BUILDDIR)
	  $(RUSTC) $(RUSTFLAGS) --test $(SRC)/main.rs -o $(TEST_TARGET)
	  ./$(TEST_TARGET) $(ARGS)

clippy: $(FAT)
	  	   @mkdir -p $(BUILDDIR)
		   $(CLIPPY) $(CLIPPYFLAGS) $(SRC)/main.rs --crate-type=bin --out-dir $(BUILDDIR)

clean:
		rm -rf $(BUILDDIR)
		rm -f  $(FAT)
		rm -f  $(FAT_OBJ)
