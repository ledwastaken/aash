NAME = aash
SRC = src/main.rs

RUSTC = rustc
DEBUG_FLAGS = -C debuginfo=2

all: debug

debug:
	$(RUSTC) $(DEBUG_FLAGS) $(SRC) -o $(NAME)

clean:
	rm -f $(NAME)

.PHONY: all clean
