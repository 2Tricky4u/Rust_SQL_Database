use crate::tokenizer::{Token, Tokenizer};

#[derive(Debug)]
pub enum Query {
    Select { columns: Vec<String>, table: String, condition: Option<String> },
    Insert { table: String, columns: Vec<String>, values: Vec<String> },
    Update { table: String, assignments: Vec<(String, String)>, condition: Option<String> },
    Delete { table: String, condition: Option<String> },
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, position: 0 }
    }

    pub fn parse(&mut self) -> Result<Query, String> {
        match self.tokens.get(self.position) {
            Some(Token::Select) => self.parse_select(),
            Some(Token::Insert) => self.parse_insert(),
            Some(Token::Update) => self.parse_update(),
            Some(Token::Delete) => self.parse_delete(),
            _ => Err("Invalid SQL command".to_string()),
        }
    }

    fn parse_select(&mut self) -> Result<Query, String> {
        self.position += 1; // Skip 'SELECT'
        let columns = self.parse_columns()?;
        self.position += 1; // Skip 'FROM'
        let table = self.parse_identifier()?;
        let condition = if self.position < self.tokens.len() && self.tokens[self.position] == Token::Where {
            self.position += 1; // Skip 'WHERE'
            Some(self.parse_identifier()?)
        } else {
            None
        };
        Ok(Query::Select { columns, table, condition })
    }

    fn parse_insert(&mut self) -> Result<Query, String> {
        self.position += 1; // Skip 'INSERT'
        self.position += 1; // Skip 'INTO'
        let table = self.parse_identifier()?;
        self.position += 1; // Skip '('
        let columns = self.parse_columns()?;
        self.position += 1; // Skip ')'
        self.position += 1; // Skip 'VALUES'
        self.position += 1; // Skip '('
        let values = self.parse_values()?;
        self.position += 1; // Skip ')'
        Ok(Query::Insert { table, columns, values })
    }

    fn parse_update(&mut self) -> Result<Query, String> {
        self.position += 1; // Skip 'UPDATE'
        let table = self.parse_identifier()?;
        self.position += 1; // Skip 'SET'
        let assignments = self.parse_assignments()?;
        let condition = if self.position < self.tokens.len() && self.tokens[self.position] == Token::Where {
            self.position += 1; // Skip 'WHERE'
            Some(self.parse_identifier()?)
        } else {
            None
        };
        Ok(Query::Update { table, assignments, condition })
    }

    fn parse_delete(&mut self) -> Result<Query, String> {
        self.position += 1; // Skip 'DELETE'
        self.position += 1; // Skip 'FROM'
        let table = self.parse_identifier()?;
        let condition = if self.position < self.tokens.len() && self.tokens[self.position] == Token::Where {
            self.position += 1; // Skip 'WHERE'
            Some(self.parse_identifier()?)
        } else {
            None
        };
        Ok(Query::Delete { table, condition })
    }

    fn parse_columns(&mut self) -> Result<Vec<String>, String> {
        let mut columns = Vec::new();
        while self.position < self.tokens.len() {
            if let Token::Identifier(ref col) = self.tokens[self.position] {
                columns.push(col.clone());
            } else {
                break;
            }
            self.position += 1;
            if self.position < self.tokens.len() && self.tokens[self.position] == Token::Comma {
                self.position += 1; // Skip ','
            } else {
                break;
            }
        }
        Ok(columns)
    }

    fn parse_values(&mut self) -> Result<Vec<String>, String> {
        let mut values = Vec::new();
        while self.position < self.tokens.len() {
            if let Token::Literal(ref val) = self.tokens[self.position] {
                values.push(val.clone());
            } else {
                break;
            }
            self.position += 1;
            if self.position < self.tokens.len() && self.tokens[self.position] == Token::Comma {
                self.position += 1; // Skip ','
            } else {
                break;
            }
        }
        Ok(values)
    }

    fn parse_assignments(&mut self) -> Result<Vec<(String, String)>, String> {
        let mut assignments = Vec::new();
        while self.position < self.tokens.len() {
            if let Token::Identifier(ref col) = self.tokens[self.position] {
                self.position += 1; // Skip column name
                self.position += 1; // Skip '='
                if let Token::Literal(ref val) = self.tokens[self.position] {
                    assignments.push((col.clone(), val.clone()));
                } else {
                    return Err("Expected literal value".to_string());
                }
            } else {
                break;
            }
            self.position += 1;
            if self.position < self.tokens.len() && self.tokens[self.position] == Token::Comma {
                self.position += 1; // Skip ','
            } else {
                break;
            }
        }
        Ok(assignments)
    }

    fn parse_identifier(&mut self) -> Result<String, String> {
        if let Some(Token::Identifier(ref ident)) = self.tokens.get(self.position) {
            self.position += 1;
            Ok(ident.clone())
        } else {
            Err("Expected identifier".to_string())
        }
    }
}
