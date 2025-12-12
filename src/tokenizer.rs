#[derive(Debug, Clone)]
pub enum OpCode {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    ShowAscii,
    StockIn,
    StartLoop,
    EndLoop,
    Error,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub instruction: String,
    pub opcode: OpCode,
    pub position: usize,
}

pub fn get_token_opcode(instruction: char) -> OpCode {
    match instruction {
        '>' => OpCode::MoveRight,
        '<' => OpCode::MoveLeft,
        '+' => OpCode::Increment,
        '-' => OpCode::Decrement,
        '.' => OpCode::ShowAscii,
        ',' => OpCode::StockIn,
        '[' => OpCode::StartLoop,
        ']' => OpCode::EndLoop,
        _ => OpCode::Error,
    }
}

impl Token {
    pub fn new(instruction: &str, opcode: OpCode, position: usize) -> Token {
        Token {
            instruction: instruction.to_string(),
            opcode,
            position,
        }
    }

    pub fn tokenize(instructions: &str) -> Vec<Token> {
        let mut tokens = Vec::new();

        for (position, instruction) in instructions.chars().enumerate() {
            let token_type = get_token_opcode(instruction);
            let current_token = Token::new(&instruction.to_string(), token_type, position);
            tokens.push(current_token);
        }
        tokens
    }
}
