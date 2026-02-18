#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Word(String),
    Punctuation(char),
}

pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Parser
    }

    pub fn tokenize(&self, input: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut current_word = String::new();

        for ch in input.chars() {
            if ch.is_whitespace() {
                if !current_word.is_empty() {
                    tokens.push(Token::Word(current_word.clone()));
                    current_word.clear();
                }
            } else if ch == '.' || ch == ',' || ch == '?' || ch == '!' || ch == ':' || ch == ';' {
                if !current_word.is_empty() {
                    tokens.push(Token::Word(current_word.clone()));
                    current_word.clear();
                }
                tokens.push(Token::Punctuation(ch));
            } else {
                current_word.push(ch);
            }
        }

        if !current_word.is_empty() {
            tokens.push(Token::Word(current_word));
        }

        tokens
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}
