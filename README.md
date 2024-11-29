
# Ika

### Ika is a purely handwritten programming language which I designed to learn how to write compilers in Rust.

## Features
The ika compiler compiles the Ika source code file into an LLVM intermediate representation and then compiles it into an executable file

## Download 

```
git clone git@github.com:Quaestiox/Ika.git
```


## Usage

Since the compiler uses LLVM IR as the intermediate code, you need to make sure you have Clang before using it (not compatible with very old versions now): [Download LLVM here](https://github.com/llvm/llvm-project/releases)


compile the Ika source file:
```
ika [-o a.out] hello.ika
```

help command:
```
ika -h
```

show the AST:
```
ika -a
```

show the tokens:
```
ika -t
```

show the source code:
```
ika -s
```


