NAME = aash
SRC = src/main.rs

RUSTC = rustc

all: $(NAME)

$(NAME): $(SRC)
	$(RUSTC) $(SRC) -o $(NAME)

clean:
	rm -f $(NAME)

re: clean all

.PHONY: all clean re
