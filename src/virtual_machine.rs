use crate::parser::build_loop_map;
use crate::tokenizer::*;
use std::io::{self};

#[derive(Debug)]
pub struct VirtualMachine {
    tape: Vec<u8>,
    ptr: usize,
}

#[warn(dead_code)]
impl VirtualMachine {
    pub fn new(size: usize) -> VirtualMachine {
        VirtualMachine {
            tape: vec![0; size],
            ptr: 0,
        }
    }

    //pub fn show(&self) {
    //    println!("{:?}", self.tape);
    //}

    pub fn run(&mut self, tokens: Vec<Token>) -> &Vec<u8> {
        let loop_map = build_loop_map(&tokens);
        let mut i: usize = 0;
        let size: usize = tokens.len();

        while i < size {
            
            match tokens[i].opcode {
                OpCode::MoveRight => {
                    if self.ptr == self.tape.len() - 1 {
                        panic!("Tape overflow towards the right");
                    }
                    self.ptr += 1;
                    i += 1
                }
                OpCode::MoveLeft => {
                    if self.ptr == 0 {
                        panic!("Tape overflow towards the left");
                    }
                    self.ptr -= 1;
                    i += 1
                }
                OpCode::Increment => {
                    self.tape[self.ptr] += 1;
                    i += 1
                }
                OpCode::Decrement => {
                    self.tape[self.ptr] -= 1;
                    i += 1
                }
                OpCode::ShowAscii => {
                    print!("{}", self.tape[self.ptr] as char);
                    i += 1
                }

                OpCode::StockIn => {
                    let mut get_in = String::new();
                    let mut buffer = vec![0];
                    io::stdin().read_line(&mut get_in).expect("Error");
                    buffer[0] = get_in.trim().parse().expect("Error");
                    self.tape[self.ptr] = buffer[0];
                    i += 1
                }

                OpCode::StartLoop => {
                    if self.tape[self.ptr] == 0 {
                        i = loop_map[i].end + 1;
                    }else{
                        i += 1;
                    }
                }

                OpCode::EndLoop => {
                    if self.tape[self.ptr] != 0 {
                        i = loop_map[i].start;
                    }else{
                        i +=1;
                    }
                }

                OpCode::Error => {
                    println!(
                        "Instruction number : {}, {} is Ignored.",
                        tokens[i].position, tokens[i].instruction,
                    );
                    i += 1
                }
            }
        }
        &self.tape
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run() {
        let tokens_test = vec![
            Token {
                instruction: "+".to_string(),
                opcode: OpCode::Increment,
                position: 0,
            },
            Token {
                instruction: ">".to_string(),
                opcode: OpCode::MoveRight,
                position: 1,
            },
            Token {
                instruction: "+".to_string(),
                opcode: OpCode::Increment,
                position: 2,
            },
            Token {
                instruction: "+".to_string(),
                opcode: OpCode::Increment,
                position: 3,
            },
            Token {
                instruction: "+".to_string(),
                opcode: OpCode::Increment,
                position: 4,
            },
            Token {
                instruction: ">".to_string(),
                opcode: OpCode::MoveRight,
                position: 5,
            },
            Token {
                instruction: "+".to_string(),
                opcode: OpCode::Increment,
                position: 6,
            },
            Token {
                instruction: "<".to_string(),
                opcode: OpCode::MoveLeft,
                position: 7,
            },
            Token {
                instruction: "<".to_string(),
                opcode: OpCode::MoveLeft,
                position: 8,
            },
            Token {
                instruction: "-".to_string(),
                opcode: OpCode::Decrement,
                position: 9,
            },
        ];

        let mut vm = VirtualMachine::new(3);
        assert_eq!(*vm.run(tokens_test), vec![0, 3, 1])
    }

    #[test]
    fn test_run_with_error() {
        let tokens_test = vec![
            Token {
                instruction: "+".to_string(),
                opcode: OpCode::Increment,
                position: 0,
            },
            Token {
                instruction: "+".to_string(),
                opcode: OpCode::Increment,
                position: 1,
            },
            Token {
                instruction: "E".to_string(),
                opcode: OpCode::Error,
                position: 2,
            },
            Token {
                instruction: "-".to_string(),
                opcode: OpCode::Decrement,
                position: 3,
            },
            Token {
                instruction: "-".to_string(),
                opcode: OpCode::Decrement,
                position: 4,
            },
        ];

        let mut vm = VirtualMachine::new(3);
        assert_eq!(*vm.run(tokens_test), vec![0, 0, 0])
    }
}