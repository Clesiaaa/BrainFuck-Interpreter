use crate::tokenizer::*;

#[derive(Debug, PartialEq, Clone)]
pub struct LoopIndex {
    pub start: usize,
    pub end: usize,
}

impl LoopIndex {
    pub fn new(start: usize, end: usize) -> Self {
        LoopIndex { start, end }
    }
}

pub fn build_loop_map(tokens: &Vec<Token>) -> Vec<LoopIndex> {
    let mut loop_map: Vec<LoopIndex> = vec![LoopIndex::new(0, 0); tokens.len()];
    let mut stack: Vec<usize> = Vec::new();

    for (i, token) in tokens.iter().enumerate() {
        match &token.opcode {
            OpCode::StartLoop => {
                stack.push(i);
            }

            OpCode::EndLoop => {
                let start = stack.pop().expect("Error Loop does not have an end.");
                loop_map[start].end = i;
                loop_map[i].start = start;
            }

            _ => {}
        }
    }
    loop_map
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_build_one_loop() {
        let tokens_test = vec![
            Token {
                instruction: "+".to_string(),
                opcode: OpCode::Increment,
                position: 0,
            },
            Token {
                instruction: "[".to_string(),
                opcode: OpCode::StartLoop,
                position: 1,
            },
            Token {
                instruction: "E".to_string(),
                opcode: OpCode::Error,
                position: 2,
            },
            Token {
                instruction: "]".to_string(),
                opcode: OpCode::EndLoop,
                position: 3,
            },
            Token {
                instruction: "-".to_string(),
                opcode: OpCode::Decrement,
                position: 4,
            },
        ];

        let loop_map = build_loop_map(&tokens_test);

        assert_eq!(loop_map, vec![LoopIndex { start: 1, end: 3 }])
    }

    #[test]
    fn test_build_duo_loop() {
        let tokens_test = vec![
            Token {
                instruction: "+".to_string(),
                opcode: OpCode::Increment,
                position: 0,
            },
            Token {
                instruction: "[".to_string(),
                opcode: OpCode::StartLoop,
                position: 1,
            },
            Token {
                instruction: "[".to_string(),
                opcode: OpCode::StartLoop,
                position: 2,
            },
            Token {
                instruction: "E".to_string(),
                opcode: OpCode::Error,
                position: 3,
            },
            Token {
                instruction: "]".to_string(),
                opcode: OpCode::EndLoop,
                position: 4,
            },
            Token {
                instruction: "]".to_string(),
                opcode: OpCode::EndLoop,
                position: 5,
            },
            Token {
                instruction: "-".to_string(),
                opcode: OpCode::Decrement,
                position: 6,
            },
        ];

        let loop_map = build_loop_map(&tokens_test);

        assert_eq!(
            loop_map,
            vec![
                LoopIndex { start: 2, end: 4 },
                LoopIndex { start: 1, end: 5 }
            ]
        )
    }
}
