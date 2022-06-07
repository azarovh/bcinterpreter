# bcinterpreter

It is a test bytecode interpreter written in Rust. 

Interpreter itself is stack-based (for now stack is limited to 16MB). The model does not support scopes, functions or branching beside a loop. Though local store is ready for implementation of scopes for variables.    
Following limited number of instructions are implemented:

| name                    | args | description |
|:------------------------|:------------:|:-------------|
| LOAD_VAL | ARG1 | Load constant value on top of the stack |
| READ_VAR | ARG1 | Store value from the top of the stack into local store with name associated |
| WRITE_VAR | ARG1 | Load value from the local store on top of the stack |
| ADD/SUBSTRACT/MULTIPLY/DEVIDE | n/a | Load 2 value from the top of the stack and apply +-*/ arithmetic operation|  
| JUMP_IF_EQ | n/a | Load two values from the stack; if they are equal execution jumps right after the nearest END instruction; otherwise the next instruction is executed |
| END | n/a | The execution jumps 2 instructions before previous JUMP_IF_* |
| RETURN_VALUE | n/a | Load value from the top of the stack and return it as a result of the program |

### Usage

Bytecode can be read from the file. File is provided as an argument:
```shell
./bcinterpreter bytecode.txt
```
