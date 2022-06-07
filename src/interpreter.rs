use crate::error::Error;
use crate::vm::VM;

// 'Interpreter'
// The interpreter is stateful and is not supposed to be run more than once.
#[derive(Default)]
pub struct Interpreter {
    bytecode: Vec<String>,
    vm: VM,
}

impl Interpreter {
    // Creates new Interpreter from the bytecode provided.
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

    // Execute the bytecode returning a value or an error.
    pub fn run(&mut self) -> Result<i32, Error> {
        let mut line = 0;
        let mut last_jump_line: Option<usize> = None;

        // iterate over instructions line by line
        while line < self.bytecode.len() {
            let instruction: Vec<&str> = self.bytecode[line].split_whitespace().collect();
            if !instruction.is_empty() {
                match instruction[0] {
                    "LOAD_VAL" => execute_unary(&instruction, |i| self.vm.load_value(i))?,
                    "WRITE_VAR" => execute_unary(&instruction, |i| self.vm.write_var(i))?,
                    "READ_VAR" => execute_unary(&instruction, |i| self.vm.read_var(i))?,
                    "ADD" => self.vm.add()?,
                    "SUBTRACT" => self.vm.sub()?,
                    "MULTIPLY" => self.vm.mul()?,
                    "DIVIDE" => self.vm.div()?,
                    "RETURN_VALUE" => return self.vm.ret(),
                    "JUMP_IF_EQ" => {
                        last_jump_line = Some(line);
                        let eq = self.vm.is_eq()?;
                        if eq {
                            //jump after nearest END
                            let end_line = self
                                .bytecode
                                .iter_mut()
                                .skip(line)
                                .position(|v| *v == "END")
                                .ok_or(Error::Syntax("No END statement for a loop".to_string()))?;
                            line += end_line;
                        }
                        //do nothing
                    }
                    "END" => {
                        //jump 2 lines before previous JUMP_IF*
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

// Helper function to safely execute unary instruction
fn execute_unary<F>(instruction: &Vec<&str>, mut f: F) -> Result<(), Error>
where
    F: FnMut(&str) -> Result<(), Error>,
{
    if instruction.len() == 2 {
        f(instruction[1])
    } else {
        Err(Error::Syntax(format!(
            "{}: invalid number of arguments - 1 required, but {} provided",
            instruction[0],
            instruction.len()
        )))
    }
}
