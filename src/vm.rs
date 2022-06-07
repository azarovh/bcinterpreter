use arrayvec::ArrayVec;

use crate::error::Error;

// 'VM' contains all the business logic for the instruction set.
// Also it stores the results of the execution.
//
// Note: 'VM' is an implementation detail and is not supposed to be used outside its create
#[derive(Default)]
pub(crate) struct VM {
    // 'stack' stores all the temporary values between commands execution
    stack: ArrayVec<i32, 512>, //TODO: make the size customizable

    // 'local' represents local storage for named variables. Vec is chosen as a natural representation of variables on the stack.
    // Thus it would be easier to implement scopes in the future.
    local: Vec<Var>,
}

struct Var {
    name: String,
    val: i32,
}

fn unquote_var(varname: &str) -> Option<&str> {
    if varname.chars().count() < 3
        || varname.chars().next().unwrap() != '\''
        || varname.chars().last().unwrap() != '\''
    {
        None
    } else {
        Some(&varname[1..varname.len() - 1])
    }
}

fn pop2_from_stack(stack: &mut ArrayVec<i32, 512>) -> Result<(i32, i32), Error> {
    let v1 = stack
        .pop()
        .ok_or(Error::Internal("Stack is empty".to_string()))?;
    let v2 = stack
        .pop()
        .ok_or(Error::Internal("Stack is empty".to_string()))?;

    return Ok((v1, v2));
}

impl VM {
    pub(crate) fn load_value(&mut self, val: &str) -> Result<(), Error> {
        match val.parse::<i32>() {
            Ok(v) => {
                if let Err(_) = self.stack.try_push(v) {
                    return Err(Error::StackOverflow);
                }
            }
            Err(_) => return Err(Error::Syntax("Could not parse the argument".to_string())),
        };
        Ok(())
    }

    pub(crate) fn write_var(&mut self, varname: &str) -> Result<(), Error> {
        let name = unquote_var(varname).ok_or(Error::Syntax(format!(
            "Could not parse the name of the var: {}",
            varname
        )))?;

        match self.local.iter_mut().find(|v| v.name == name) {
            Some(v) => v.val = self.stack.pop().unwrap(),
            None => {
                self.local.push(Var {
                    name: name.to_string(),
                    val: self.stack.pop().unwrap(),
                });
            }
        }
        Ok(())
    }

    pub(crate) fn read_var(&mut self, varname: &str) -> Result<(), Error> {
        let name = unquote_var(varname).ok_or(Error::Syntax(format!(
            "Could not parse the name of the var: {}",
            varname
        )))?;
        match self.local.iter().find(|v| v.name == name) {
            Some(v) => {
                if let Err(_) = self.stack.try_push(v.val) {
                    return Err(Error::StackOverflow);
                }
            }
            None => {
                return Err(Error::UndefinedVar(format!(
                    "Undefined variable: {}",
                    varname
                )));
            }
        }
        Ok(())
    }

    pub(crate) fn add(&mut self) -> Result<(), Error> {
        let (left, right) = pop2_from_stack(&mut self.stack)?;

        if let Err(_) = self.stack.try_push(left + right) {
            Err(Error::StackOverflow)
        } else {
            Ok(())
        }
    }

    pub(crate) fn sub(&mut self) -> Result<(), Error> {
        let (left, right) = pop2_from_stack(&mut self.stack)?;

        if let Err(_) = self.stack.try_push(left - right) {
            Err(Error::StackOverflow)
        } else {
            Ok(())
        }
    }

    pub(crate) fn mul(&mut self) -> Result<(), Error> {
        let (left, right) = pop2_from_stack(&mut self.stack)?;

        if let Err(_) = self.stack.try_push(left * right) {
            Err(Error::StackOverflow)
        } else {
            Ok(())
        }
    }

    pub(crate) fn div(&mut self) -> Result<(), Error> {
        let (left, right) = pop2_from_stack(&mut self.stack)?;

        if let Err(_) = self.stack.try_push(left / right) {
            Err(Error::StackOverflow)
        } else {
            Ok(())
        }
    }

    pub(crate) fn ret(&mut self) -> Result<i32, Error> {
        self.stack
            .pop()
            .ok_or(Error::Internal("Stack is empty".to_string()))
    }

    pub(crate) fn is_eq(&mut self) -> Result<bool, Error> {
        let (left, right) = pop2_from_stack(&mut self.stack)?;
        Ok(left == right)
    }
}
