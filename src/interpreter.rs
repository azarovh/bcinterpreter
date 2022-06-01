use crate::error::Error;
use crate::vm::VM;

#[derive(Default)]
pub struct Interpreter {
    bytecode: Vec<String>,
    vm: VM,
}

impl Interpreter {
    pub fn new(bytecode: &str) -> Interpreter {
        Interpreter {
            bytecode: bytecode
                .lines()
                .filter(|v| !v.is_empty())
                .map(|v| v.trim().to_string())
                .collect(),
            ..Default::default()
        }
    }

    pub fn run(&mut self) -> Result<i32, Error> {
        let mut line = 0;
        let mut last_jump_line: Option<usize> = None;
        while line < self.bytecode.len() {
            let instruction: Vec<&str> = self.bytecode[line].split_whitespace().collect();
            if !instruction.is_empty() {
                eprintln!("{:?}", instruction);
                match instruction[0] {
                    "LOAD_VAL" => {
                        if instruction.len() == 2 {
                            self.vm.load_value(instruction[1])?;
                        } else {
                            return Err(Error::Syntax(format!(
                                "{}: invalid number of arguments - 1 required, but {} provided",
                                instruction[0],
                                instruction.len()
                            )));
                        }
                    }
                    "WRITE_VAR" => {
                        if instruction.len() == 2 {
                            self.vm.write_var(instruction[1])?;
                        } else {
                            return Err(Error::Syntax(format!(
                                "{}: invalid number of arguments - 1 required, but {} provided",
                                instruction[0],
                                instruction.len()
                            )));
                        }
                    }
                    "READ_VAR" => {
                        if instruction.len() == 2 {
                            self.vm.read_var(instruction[1])?;
                        } else {
                            return Err(Error::Syntax(format!(
                                "{}: invalid number of arguments - 1 required, but {} provided",
                                instruction[0],
                                instruction.len()
                            )));
                        }
                    }
                    "ADD" => self.vm.add()?,
                    "SUBTRACT" => self.vm.sub()?,
                    "MULTIPLY" => self.vm.mul()?,
                    "DIVIDE" => self.vm.div()?,
                    "RETURN_VALUE" => return self.vm.ret(),
                    "JUMP_IF_EQ" => {
                        last_jump_line = Some(line);
                        let eq = self.vm.is_eq()?;
                        if eq {
                            let end_line = self
                                .bytecode
                                .iter_mut()
                                .skip(line)
                                .position(|v| *v == "END")
                                .ok_or(Error::Syntax("No END statement for a loop".to_string()))?;
                            line += end_line;
                        }
                    }
                    "END" => {
                        line = last_jump_line
                            .ok_or(Error::Syntax("No JUMP* statement for a loop".to_string()))?;
                        line -= 2;
                        continue;
                    }
                    _ => {
                        return Err(Error::InvalidOp(format!(
                            "Unknown operation: {}",
                            instruction[0]
                        )))
                    }
                };
            }
            line += 1;
        }

        Ok(0)
    }
}
