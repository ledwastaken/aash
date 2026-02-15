NAME = aash
SRC = src/main.rs

RUSTC = rustc
DEBUG_FLAGS = -C debuginfo=2
RELEASE_FLAGS = -O -C debuginfo=0

all: debug

debug:
	$(RUSTC) $(DEBUG_FLAGS) $(SRC) -o $(NAME)

release:
	$(RUSTC) $(RELEASE_FLAGS) $(SRC) -o $(NAME)

clean:
	rm -f $(NAME)

.PHONY: all debug release clean
