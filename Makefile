NAME=emtor
RUSTC=rustc
RUSTCFLAGS=
SRC=src
BUILDDIR=target
TARGET=$(BUILDDIR)/$(NAME)
SRCS=$(shell find $(SRC) -name '*.rs')

.PHONY: all clean

all: $(TARGET)

$(TARGET): $(SRCS)
	@mkdir -p $(BUILDDIR)
	$(RUSTC) $(RUSTFLAGS) $(SRC)/main.rs -o $(TARGET)

clean: rm -rf $(BUILDDIR)
