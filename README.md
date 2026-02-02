# aash

`aash` is a POSIX-compliant Unix shell implementation written in Rust.

The goal of this project is to provide a correct and lightweight shell, focusing on POSIX behavior, portability, and a clean internal design.

## Usage

Run commands from a string using the `-c` option:
```bash
sh$ ./aash -c "echo Input as string"
Input as string
```

Run commands from a file by passing the file path as a positional argument:
```bash
sh$ cat -e script.sh
echo Input as file$
sh$ ./aash script.sh
Input as file
```

If no `-c` option and no script file are provided, `aash` reads commands from standard input:
```bash
sh$ cat -e script.sh
echo Input through stdin$
sh$ ./aash < script.sh
Input through stdin
sh$ cat script.sh | ./aash
Input through stdin
```
