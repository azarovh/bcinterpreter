extern crate bcinterpreter;

use bcinterpreter::Error;
use bcinterpreter::Interpreter;

#[test]
fn load_val() {
    let bytecode = "LOAD_VAL 1
            			RETURN_VALUE";
    let mut ip = Interpreter::new(bytecode);
    assert_eq!(1, ip.run().unwrap());
}

#[test]
fn failed_load_val() {
    let bytecode = "LOAD_VAL f
		            	RETURN_VALUE";
    let mut ip = Interpreter::new(bytecode);
    let err = ip.run().unwrap_err();
    assert!(matches!(err, Error::Syntax { .. }));
}

#[test]
fn sum_2_values() {
    let bytecode = "LOAD_VAL 1
    	                LOAD_VAL 2
            			ADD
            			RETURN_VALUE";
    let mut ip = Interpreter::new(bytecode);
    assert_eq!(3, ip.run().unwrap());
}

#[test]
fn read_variable() {
    let bytecode = "LOAD_VAL 1
    	                WRITE_VAR 'x'
    	                READ_VAR 'x'

            			RETURN_VALUE";
    let mut ip = Interpreter::new(bytecode);
    assert_eq!(1, ip.run().unwrap());
}

#[test]
fn example() {
    let bytecode = "LOAD_VAL 1
                        WRITE_VAR 'x'

       	                LOAD_VAL 2
    	                WRITE_VAR 'y'

    	                READ_VAR 'x'
    	                LOAD_VAL 1
    	                ADD

    	                READ_VAR 'y'
    	                MULTIPLY
			
            			RETURN_VALUE";
    let mut ip = Interpreter::new(bytecode);
    assert_eq!(4, ip.run().unwrap());
}

#[test]
fn simple_loop() {
    let bytecode = "LOAD_VAL 1
                        WRITE_VAR 'x'

            			LOAD_VAL 5
                        READ_VAR 'x'

    	                JUMP_IF_EQ
    			        LOAD_VAL 1
                        READ_VAR 'x'
            			ADD
                        WRITE_VAR 'x'
            			END

                        READ_VAR 'x'
            			RETURN_VALUE";
    let mut ip = Interpreter::new(bytecode);
    assert_eq!(5, ip.run().unwrap());
}

#[test]
fn broken_loop() {
    let bytecode = "JUMP_IF_EQ
			            LOAD_VAL 1
                        READ_VAR 'x'
			            ADD
                        WRITE_VAR 'x'

            			RETURN_VALUE";
    let mut ip = Interpreter::new(bytecode);
    let err = ip.run().unwrap_err();
    assert!(matches!(err, Error::Internal { .. }));
}

#[test]
fn random_input() {
    let bytecode = "sldfj39h7#&";
    let mut ip = Interpreter::new(bytecode);
    let err = ip.run().unwrap_err();
    assert!(matches!(err, Error::InvalidOp { .. }));
}

#[test]
fn undefined_var() {
    let bytecode = "READ_VAR 'x'";
    let mut ip = Interpreter::new(bytecode);
    let err = ip.run().unwrap_err();
    assert!(matches!(err, Error::UndefinedVar { .. }));
}

#[test]
fn stackoverflow() {
    let mut bytecode = String::new();
    for _ in 0..1000 {
        bytecode.push_str("LOAD_VAL 1\n");
    }

    let mut ip = Interpreter::new(&bytecode);
    let err = ip.run().unwrap_err();
    assert!(matches!(err, Error::StackOverflow { .. }));
}
