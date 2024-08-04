use crate::tokenizer::{Token, Tokenizer};

#[derive(Debug)]
pub enum Query {
    Select { columns: Vec<String>, table: String, condition: Option<String>},
    Insert { table: String, columns: Vec<String>, values: Vec<String> },
    Update { table: String, assignments: Vec<(String, String)>, condition: Option<String>},
    Delete {table: String, condition: Option<String>},
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser{ tokens, position: 0}
    }

    pub fn parse(&mut self) -> Result<Query, String> {
        match self.tokens.get(self.position) {
            Some(Token::Select) => self.parse_select(),
            Some(Token::Insert) => self.parse_insert(),
            Some(Token::Update) => self.parse_update(),
            Some(Token::Delete) => self.parse_delete(),
            _ => Err("Invalid SQL command or unsupported one!".to_string()),
        }
    }

    fn parse_select(&mut self) -> Result<Query, String> {
        self.position += 1; // Skip "SELECT"
        let columns = self.parse_columns()?;
        self.position += 1; //Skip "From"
        let table = self.parse_identifier()?;
        let condition = if self.position < self.tokens.len()
                            && self.tokens[self.position] == Token::Where {
            self.position += 1; //Skip "WHERE"
            Some(self.parse_identifier()?)
        } else {
            None
        };
        Ok(Query::Select {columns, table, condition})
    }

    fn parse_update(&mut self) -> Result<Query, String> {
        self.position += 1; //Skip "UPDATE"
        let table = self.parse_identifier()?;
        self.position += 1; //Skip "SET"
        let assignments = self.parse_assignments()?;
        let condition = if self.position < self.tokens.len()
                            && self.tokens[self.position] == Token::Where {
            self.position += 1; //Skip "WHERE"
            Some(self.parse_identifier()?)
        } else {
            None
        };
        Ok(Query::Update {table, assignments, condition})
    }

}