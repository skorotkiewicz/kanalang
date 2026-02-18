use crate::dict::Dictionary;
use crate::parser::Parser;

pub struct Translator {
    dict: Dictionary,
    parser: Parser,
}

impl Translator {
    pub fn new() -> Self {
        let dict = Dictionary::new();
        let parser = Parser::new(dict.clone());
        Translator { dict, parser }
    }

    pub fn english_to_kana(&self, input: &str) -> String {
        let tokens = self.parser.tokenize(input);
        let mut result = Vec::new();
        let mut has_subject = false;
        let mut has_verb = false;
        let mut has_object = false;
        let mut is_question = false;

        if input.trim().ends_with('?') {
            is_question = true;
        }

        if input.to_lowercase().starts_with("hello")
            || input.to_lowercase().starts_with("hi ")
            || input.to_lowercase().starts_with("hey")
        {
            result.push("yu".to_string());
            return result.join(" ");
        }

        let mut pending_modifiers: Vec<String> = Vec::new();

        for token in &tokens {
            if let crate::parser::Token::Word(word) = token {
                let lower = word.to_lowercase();

                if lower == "and" {
                    continue;
                } else if lower == "not" || lower == "no" || lower == "don't" {
                    pending_modifiers.push("ala".to_string());
                } else if lower == "the" || lower == "a" || lower == "an" {
                    continue;
                } else if lower == "very" || lower == "really" {
                    pending_modifiers.push("mute".to_string());
                } else if !has_subject {
                    let kana: String = if lower == "i"
                        || lower == "me"
                        || lower == "we"
                        || lower == "us"
                    {
                        "mi".to_string()
                    } else if lower == "you" {
                        "sina".to_string()
                    } else if lower == "he" || lower == "she" || lower == "it" || lower == "they" {
                        "ona".to_string()
                    } else if let Some(k) = self.dict.translate_en_word(&lower) {
                        k.to_string()
                    } else {
                        format!("[{}]", word)
                    };
                    result.push(kana);
                    has_subject = true;
                } else if !has_verb {
                    if !result.contains(&"li".to_string())
                        && !result
                            .last()
                            .map(|w| w == "sina" || w == "mi")
                            .unwrap_or(false)
                    {
                        result.push("li".to_string());
                    }
                    if let Some(kana) = self.dict.translate_en_word(&lower) {
                        result.push(kana.to_string());
                    } else {
                        result.push(format!("[{}]", word));
                    }
                    has_verb = true;
                } else {
                    let (kana, is_pronoun) = if lower == "i"
                        || lower == "me"
                        || lower == "we"
                        || lower == "us"
                    {
                        ("mi", true)
                    } else if lower == "you" {
                        ("sina", true)
                    } else if lower == "he" || lower == "she" || lower == "it" || lower == "they" {
                        ("ona", true)
                    } else if let Some(k) = self.dict.translate_en_word(&lower) {
                        (k, false)
                    } else {
                        (lower.as_str(), false)
                    };

                    let is_entity = is_pronoun
                        || matches!(
                            self.dict.get_word_type(kana),
                            Some(crate::dict::WordType::Entity)
                                | Some(crate::dict::WordType::Number)
                        );

                    if is_entity && !has_object {
                        result.push("e".to_string());
                        has_object = true;
                    }

                    if kana == lower
                        && !is_pronoun
                        && !self.dict.translate_en_word(&lower).is_some()
                    {
                        result.push(format!("[{}]", word));
                    } else {
                        result.push(kana.to_string());
                    }
                }
            }
        }

        for modifier in pending_modifiers {
            result.push(modifier);
        }

        if is_question {
            result.insert(0, "se".to_string());
        }

        if result.is_empty() {
            return input.to_string();
        }

        result.join(" ")
    }

    pub fn kana_to_english(&self, input: &str) -> String {
        let tokens = self.parser.tokenize(input);
        let mut result = Vec::new();
        let mut stage = 0;
        let mut is_question = false;
        let mut is_negated = false;
        let mut is_greeting = false;

        let lower_input = input.to_lowercase().trim().to_string();
        if lower_input == "yu" || lower_input.starts_with("yu ") || lower_input == "y" {
            is_greeting = true;
        }

        for token in &tokens {
            if let crate::parser::Token::Word(word) = token {
                let lower = word.to_lowercase();

                if lower == "yu" {
                    result.push("hello".to_string());
                    continue;
                }

                if lower == "se" {
                    is_question = true;
                    continue;
                }

                if lower == "li" {
                    stage = 1;
                    continue;
                }

                if lower == "e" {
                    stage = 2;
                    continue;
                }

                if lower == "pi" {
                    continue;
                }

                if lower == "ala" {
                    is_negated = true;
                    continue;
                }

                if lower == "en" {
                    result.push("and".to_string());
                    continue;
                }

                if let Some(meanings) = self.dict.translate_kana_word(&lower) {
                    let meaning = if !meanings.is_empty() {
                        let m = meanings[0];
                        if m.starts_with('[') {
                            ""
                        } else {
                            m
                        }
                    } else {
                        ""
                    };

                    if !meaning.is_empty() {
                        if stage == 0 {
                            if let Some(trans) = self.get_english_pronoun(&lower) {
                                result.push(trans.to_string());
                            } else {
                                result.push(meaning.to_string());
                            }
                        } else if stage == 1 {
                            if is_negated {
                                result.push(format!("do not {}", meaning));
                                is_negated = false;
                            } else {
                                result.push(meaning.to_string());
                            }
                        } else {
                            result.push(meaning.to_string());
                        }
                    }
                } else {
                    result.push(format!("[{}]", word));
                }
            }
        }

        if is_greeting {
            return "hello".to_string();
        }

        if result.is_empty() {
            return input.to_string();
        }

        let mut output = result.join(" ");

        if is_question {
            output = format!("{}?", output);
        } else {
            output = format!("{}.", output);
        }

        output
    }

    fn get_english_pronoun(&self, kana: &str) -> Option<&'static str> {
        match kana {
            "mi" => Some("I"),
            "sina" => Some("you"),
            "ona" => Some("they"),
            _ => None,
        }
    }

    pub fn translate(&self, input: &str, direction: &str) -> String {
        match direction {
            "to" | "en2k" | "en-kana" => self.english_to_kana(input),
            "from" | "k2en" | "kana-en" => self.kana_to_english(input),
            _ => {
                let lower = input.to_lowercase();
                let kana_words = [
                    "mi", "sina", "ona", "li", "e", "pona", "ike", "toki", "moku",
                ];
                let is_kana = kana_words.iter().any(|w| lower.contains(w));

                if is_kana {
                    self.kana_to_english(input)
                } else {
                    self.english_to_kana(input)
                }
            }
        }
    }
}

impl Clone for Translator {
    fn clone(&self) -> Self {
        let dict = Dictionary::new();
        let parser = Parser::new(dict.clone());
        Translator { dict, parser }
    }
}
