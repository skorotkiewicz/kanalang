use crate::dict::{Dictionary, WordType};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Word(String),
    Punctuation(char),
}

#[derive(Debug)]
pub struct ParsedSentence {
    pub subject: Vec<String>,
    pub verb: Vec<String>,
    pub object: Vec<String>,
    pub modifiers: Vec<(String, String)>,
    pub is_question: bool,
    pub is_negated: bool,
}

#[derive(Debug)]
pub enum ParseError {
    MissingSubject,
    MissingVerb,
    InvalidWord(String),
    GrammarError(String),
}

pub struct Parser {
    dict: Dictionary,
}

impl Parser {
    pub fn new(dict: Dictionary) -> Self {
        Parser { dict }
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

    pub fn parse_kana(&self, input: &str) -> Result<ParsedSentence, ParseError> {
        let tokens = self.tokenize(input);
        let mut words: Vec<String> = Vec::new();
        let mut is_question = false;

        for token in &tokens {
            match token {
                Token::Word(w) => words.push(w.clone()),
                Token::Punctuation('?') => is_question = true,
                _ => {}
            }
        }

        if words.is_empty() {
            return Err(ParseError::MissingSubject);
        }

        let mut subject = Vec::new();
        let mut verb = Vec::new();
        let mut object = Vec::new();
        let modifiers = Vec::new();
        let mut is_negated = false;

        let mut stage = 0;
        let mut i = 0;

        while i < words.len() {
            let word = &words[i];

            if word == "li" {
                stage = 1;
                i += 1;
                continue;
            }

            if word == "e" {
                stage = 2;
                i += 1;
                continue;
            }

            if word == "pi" {
                if i + 1 < words.len() {
                    i += 1;
                    continue;
                }
            }

            if word == "ala" || word == "no" {
                is_negated = true;
                i += 1;
                continue;
            }

            if word == "se" {
                is_question = true;
                i += 1;
                continue;
            }

            match stage {
                0 => subject.push(word.clone()),
                1 => verb.push(word.clone()),
                2 => object.push(word.clone()),
                _ => {}
            }

            i += 1;
        }

        if subject.is_empty() {
            return Err(ParseError::MissingSubject);
        }

        Ok(ParsedSentence {
            subject,
            verb,
            object,
            modifiers,
            is_question,
            is_negated,
        })
    }

    pub fn parse_english(&self, input: &str) -> Result<ParsedSentence, ParseError> {
        let tokens = self.tokenize(input);
        let mut words: Vec<String> = Vec::new();
        let mut is_question = false;

        for token in &tokens {
            match token {
                Token::Word(w) => words.push(w.clone()),
                Token::Punctuation('?') => is_question = true,
                _ => {}
            }
        }

        if words.is_empty() {
            return Err(ParseError::MissingSubject);
        }

        let mut subject = Vec::new();
        let mut verb = Vec::new();
        let mut object = Vec::new();
        let modifiers = Vec::new();
        let mut is_negated = false;

        let mut stage = 0;

        for word in &words {
            let lower = word.to_lowercase();

            if lower == "not" || lower == "no" || lower == "don't" || lower == "doesn't" {
                is_negated = true;
                continue;
            }

            if lower == "i"
                || lower == "we"
                || lower == "you"
                || lower == "he"
                || lower == "she"
                || lower == "it"
                || lower == "they"
            {
                if stage == 0 {
                    subject.push(word.clone());
                    stage = 1;
                }
                continue;
            }

            if let Some(wtype) = self
                .dict
                .get_word_type(self.dict.translate_en_word(&lower).unwrap_or(&lower))
            {
                match wtype {
                    WordType::Action if stage == 1 => {
                        verb.push(word.clone());
                        stage = 2;
                    }
                    WordType::Entity if stage >= 2 => {
                        object.push(word.clone());
                    }
                    _ => {}
                }
            } else if stage == 1 {
                verb.push(word.clone());
                stage = 2;
            } else if stage >= 2 {
                object.push(word.clone());
            }
        }

        if subject.is_empty() {
            subject.push("unknown".to_string());
        }

        Ok(ParsedSentence {
            subject,
            verb,
            object,
            modifiers,
            is_question,
            is_negated,
        })
    }

    pub fn validate_kana(&self, sentence: &ParsedSentence) -> Result<(), ParseError> {
        for word in &sentence.subject {
            if self.dict.translate_kana_word(word).is_none() {
                return Err(ParseError::InvalidWord(word.clone()));
            }
        }

        for word in &sentence.verb {
            if self.dict.translate_kana_word(word).is_none() {
                return Err(ParseError::InvalidWord(word.clone()));
            }
        }

        for word in &sentence.object {
            if self.dict.translate_kana_word(word).is_none() {
                return Err(ParseError::InvalidWord(word.clone()));
            }
        }

        Ok(())
    }
}
