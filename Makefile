NAME=emtor
RUSTC=rustc
RUSTCFLAGS=
SRC=src
BUILDDIR=target
TARGET=$(BUILDDIR)/$(NAME)
TEST_TARGET=$(BUILDDIR)/$(NAME)_test
SRCS=$(shell find $(SRC) -name '*.rs')

.PHONY: all clean test

all: $(TARGET)

$(TARGET): $(SRCS)
		   @mkdir -p $(BUILDDIR)
		   $(RUSTC) $(RUSTFLAGS) $(SRC)/main.rs -o $(TARGET)

clean: rm -rf $(BUILDDIR)

test: $(SRCS)
	  @mkdir -p $(BUILDDIR)
	  $(RUSTC) $(RUSTFLAGS) --test $(SRC)/main.rs -o $(TEST_TARGET)
	  ./$(TEST_TARGET)
