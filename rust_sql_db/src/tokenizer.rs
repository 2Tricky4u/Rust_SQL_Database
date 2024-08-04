use regex::Regex;

#[derive(Debug, PartialEq)]

pub enum Token {
    Select,
    Insert,
    Update,
    Delete,
    From,
    Where,
    Into,
    Values,
    Set,
    Identifier(String),
    Comma,
    SemiColon,
    Equals,
    Literal(String),
}

pub struct Tokenizer {
    input: String,
    position: usize,
}

impl Tokenizer {
    pub fn new(input: String) -> Self{
        Tokenizer{
            input,
            position: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let re = Regex::new(r"(?i)(select|insert|update|delete|from|where|into|values|set)|([a-zA-Z_][a-zA-Z0-9_]*)|(\d+)|([,;=])").unwrap();
        let mut tokens = Vec::new();

        for cap in re.captures_iter(&self.input){
            if let Some(keyword) = cap.get(1) {
                match keyword.as_str().to_lowercase().as_str() {
                    "select" => tokens.push(Token::Select),
                    "insert" => tokens.push(Token::Insert),
                    "update" => tokens.push(Token::Update),
                    "delete" => tokens.push(Token::Delete),
                    "from" => tokens.push(Token::From),
                    "where" => tokens.push(Token::Where),
                    "into" => tokens.push(Token::Into),
                    "values" => tokens.push(Token::Values),
                    "set" => tokens.push(Token::Set),
                    _ => {},
                }
            } else if let Some(identifier) = cap.get(2){
                tokens.push(Token::Identifier(identifier.as_str().to_string()));
            } else if let Some(literal) = cap.get(3) {
                tokens.push(Token::Literal(literal.as_str().to_string()));
            } else if let Some(symbol) = cap.get(4){
                match symbol.as_str() {
                    "," => tokens.push(Token::Comma),
                    ";" => tokens.push(Token::SemiColon),
                    "=" => tokens.push(Token::Equals),
                    _ => {},
                }
            }
        }

        tokens
    }
}