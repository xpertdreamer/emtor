NAME=emtor
RUSTC=rustc
RUSTCFLAGS=
CLIPPY=clippy-driver
SRC=src
BUILDDIR=target
TARGET=$(BUILDDIR)/$(NAME)
TEST_TARGET=$(BUILDDIR)/$(NAME)_test
SRCS=$(shell find $(SRC) -name '*.rs')

.PHONY: all clean test clippy

all: $(TARGET)

$(TARGET): $(SRCS)
		   @mkdir -p $(BUILDDIR)
		   $(RUSTC) $(RUSTFLAGS) $(SRC)/main.rs -o $(TARGET)

test: $(SRCS)
	  @mkdir -p $(BUILDDIR)
	  $(RUSTC) $(RUSTFLAGS) --test $(SRC)/main.rs -o $(TEST_TARGET)
	  ./$(TEST_TARGET) $(ARGS)

clippy:
		   $(CLIPPY) $(RUSTFLAGS) $(SRC)/main.rs --crate-type=bin --out-dir $(BUILDDIR)
		   ./$(BUILDDIR)/main

clean: rm -rf $(BUILDDIR)
