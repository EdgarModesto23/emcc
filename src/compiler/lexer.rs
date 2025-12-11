pub struct Lexer {
    input: String,
    position: i32,
    read_position: i32,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let first_char = input.chars().nth(0).unwrap();
        Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: first_char,
        }
    }
}
