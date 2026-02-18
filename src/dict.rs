use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum WordType {
    Entity,
    Action,
    Quality,
    Particle,
    Number,
}

#[derive(Clone)]
pub struct Word {
    pub kana: &'static str,
    pub english: Vec<&'static str>,
    pub wtype: WordType,
}

#[derive(Clone)]
pub struct Dictionary {
    pub en_to_kana: HashMap<String, &'static str>,
    pub kana_to_en: HashMap<String, Vec<&'static str>>,
    pub word_types: HashMap<String, WordType>,
}

impl Dictionary {
    pub fn new() -> Self {
        let words: Vec<Word> = vec![
            Word {
                kana: "mi",
                english: vec!["i", "me", "we", "us"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "sina",
                english: vec!["you"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "ona",
                english: vec!["he", "she", "it", "they"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "jan",
                english: vec!["person", "human", "people"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "kala",
                english: vec!["fish", "animal", "creature"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "kasi",
                english: vec!["plant", "tree", "nature", "vegetation"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "pona",
                english: vec!["good", "simple", "positive", "correct"],
                wtype: WordType::Quality,
            },
            Word {
                kana: "ike",
                english: vec!["bad", "wrong", "negative", "complex"],
                wtype: WordType::Quality,
            },
            Word {
                kana: "suli",
                english: vec!["big", "important", "long", "tall"],
                wtype: WordType::Quality,
            },
            Word {
                kana: "lili",
                english: vec!["small", "little", "short", "young"],
                wtype: WordType::Quality,
            },
            Word {
                kana: "wawa",
                english: vec!["strong", "powerful", "energy"],
                wtype: WordType::Quality,
            },
            Word {
                kana: "mute",
                english: vec!["many", "much", "very", "a lot"],
                wtype: WordType::Quality,
            },
            Word {
                kana: "sona",
                english: vec!["know", "knowledge", "wisdom", "understand"],
                wtype: WordType::Action,
            },
            Word {
                kana: "wile",
                english: vec!["want", "need", "desire", "wish"],
                wtype: WordType::Action,
            },
            Word {
                kana: "ken",
                english: vec!["can", "able", "possible", "ability"],
                wtype: WordType::Action,
            },
            Word {
                kana: "lukin",
                english: vec!["see", "look", "watch", "eye"],
                wtype: WordType::Action,
            },
            Word {
                kana: "kute",
                english: vec!["hear", "listen", "ear"],
                wtype: WordType::Action,
            },
            Word {
                kana: "toki",
                english: vec!["speak", "say", "talk", "communicate", "language"],
                wtype: WordType::Action,
            },
            Word {
                kana: "pilin",
                english: vec!["feel", "think", "believe", "emotion", "heart"],
                wtype: WordType::Action,
            },
            Word {
                kana: "moku",
                english: vec!["eat", "food", "consume"],
                wtype: WordType::Action,
            },
            Word {
                kana: "lape",
                english: vec!["sleep", "rest", "tired"],
                wtype: WordType::Action,
            },
            Word {
                kana: "pali",
                english: vec!["do", "make", "create", "work"],
                wtype: WordType::Action,
            },
            Word {
                kana: "tawa",
                english: vec!["go", "move", "to", "towards"],
                wtype: WordType::Action,
            },
            Word {
                kana: "kama",
                english: vec!["come", "arrive", "become", "future"],
                wtype: WordType::Action,
            },
            Word {
                kana: "awen",
                english: vec!["stay", "remain", "wait", "keep"],
                wtype: WordType::Action,
            },
            Word {
                kana: "weka",
                english: vec!["remove", "away", "absent", "gone"],
                wtype: WordType::Action,
            },
            Word {
                kana: "jo",
                english: vec!["have", "hold", "possess"],
                wtype: WordType::Action,
            },
            Word {
                kana: "li",
                english: vec!["[subject-marker]"],
                wtype: WordType::Particle,
            },
            Word {
                kana: "e",
                english: vec!["[object-marker]"],
                wtype: WordType::Particle,
            },
            Word {
                kana: "pi",
                english: vec!["[modifier-marker]"],
                wtype: WordType::Particle,
            },
            Word {
                kana: "la",
                english: vec!["[context-marker]", "if", "when"],
                wtype: WordType::Particle,
            },
            Word {
                kana: "anu",
                english: vec!["or"],
                wtype: WordType::Particle,
            },
            Word {
                kana: "en",
                english: vec!["and"],
                wtype: WordType::Particle,
            },
            Word {
                kana: "se",
                english: vec!["[question-marker]", "?", "what"],
                wtype: WordType::Particle,
            },
            Word {
                kana: "ala",
                english: vec!["no", "not", "none", "zero"],
                wtype: WordType::Quality,
            },
            Word {
                kana: "kin",
                english: vec!["also", "too", "indeed"],
                wtype: WordType::Particle,
            },
            Word {
                kana: "ni",
                english: vec!["this", "that", "these", "those"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "ale",
                english: vec!["all", "everything", "universe", "life"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "ijo",
                english: vec!["thing", "something", "object"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "ma",
                english: vec!["land", "world", "place", "country", "earth"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "tomo",
                english: vec!["house", "building", "room", "home"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "ilo",
                english: vec!["tool", "device", "machine"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "pana",
                english: vec!["give", "send", "release", "emit"],
                wtype: WordType::Action,
            },
            Word {
                kana: "olin",
                english: vec!["love", "affection", "respect", "care"],
                wtype: WordType::Action,
            },
            Word {
                kana: "nasin",
                english: vec!["way", "method", "path", "road", "direction"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "tenpo",
                english: vec!["time", "period", "moment", "situation"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "suno",
                english: vec!["sun", "day", "light", "brightness"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "mun",
                english: vec!["moon", "night", "star"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "telo",
                english: vec!["water", "liquid", "fluid", "drink"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "kon",
                english: vec!["air", "wind", "breath", "spirit"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "seli",
                english: vec!["fire", "heat", "warm", "cook"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "lete",
                english: vec!["cold", "ice", "cool", "frozen"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "akuta",
                english: vec!["truth", "real", "honest", "transparent"],
                wtype: WordType::Quality,
            },
            Word {
                kana: "jaki",
                english: vec!["dirty", "gross", "contaminated"],
                wtype: WordType::Quality,
            },
            Word {
                kana: "sin",
                english: vec!["new", "fresh", "another", "more"],
                wtype: WordType::Quality,
            },
            Word {
                kana: "pini",
                english: vec!["done", "finished", "completed", "past", "end"],
                wtype: WordType::Quality,
            },
            Word {
                kana: "kule",
                english: vec!["color", "colorful", "paint"],
                wtype: WordType::Quality,
            },
            Word {
                kana: "walo",
                english: vec!["white", "light-colored", "pale"],
                wtype: WordType::Quality,
            },
            Word {
                kana: "pimeja",
                english: vec!["black", "dark", "darkness"],
                wtype: WordType::Quality,
            },
            Word {
                kana: "loje",
                english: vec!["red", "reddish"],
                wtype: WordType::Quality,
            },
            Word {
                kana: "laso",
                english: vec!["blue", "green", "bluish"],
                wtype: WordType::Quality,
            },
            Word {
                kana: "jelo",
                english: vec!["yellow", "yellowish"],
                wtype: WordType::Quality,
            },
            Word {
                kana: "luka",
                english: vec!["hand", "arm", "five", "touch"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "noka",
                english: vec!["foot", "leg", "walk", "bottom"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "monsi",
                english: vec!["back", "behind", "rear"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "sinpin",
                english: vec!["front", "face", "wall"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "sewi",
                english: vec!["up", "above", "high", "sky", "divine"],
                wtype: WordType::Quality,
            },
            Word {
                kana: "anpa",
                english: vec!["down", "below", "low", "humble"],
                wtype: WordType::Quality,
            },
            Word {
                kana: "insa",
                english: vec!["inside", "inner", "center", "stomach"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "poka",
                english: vec!["side", "next-to", "hip", "with"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "lon",
                english: vec!["located-at", "exist", "real", "true"],
                wtype: WordType::Action,
            },
            Word {
                kana: "tan",
                english: vec!["from", "because-of", "origin", "cause"],
                wtype: WordType::Particle,
            },
            Word {
                kana: "sama",
                english: vec!["same", "similar", "equal", "like"],
                wtype: WordType::Quality,
            },
            Word {
                kana: "ante",
                english: vec!["different", "other", "changed"],
                wtype: WordType::Quality,
            },
            Word {
                kana: "kepeken",
                english: vec!["use", "with", "using"],
                wtype: WordType::Action,
            },
            Word {
                kana: "open",
                english: vec!["begin", "start", "open"],
                wtype: WordType::Action,
            },
            Word {
                kana: "pan",
                english: vec!["grain", "bread", "rice", "food"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "esun",
                english: vec!["trade", "market", "buy", "sell", "shop"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "lape",
                english: vec!["sleep", "rest"],
                wtype: WordType::Action,
            },
            Word {
                kana: "musi",
                english: vec!["fun", "play", "entertain", "art"],
                wtype: WordType::Action,
            },
            Word {
                kana: "uta",
                english: vec!["mouth", "lips"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "nena",
                english: vec!["bump", "hill", "mountain", "nose"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "linja",
                english: vec!["long", "rope", "hair", "line"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "palisa",
                english: vec!["stick", "rod", "long-object"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "lupa",
                english: vec!["hole", "door", "window", "opening"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "lipu",
                english: vec!["paper", "book", "document", "flat"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "kiwen",
                english: vec!["hard", "solid", "stone", "metal"],
                wtype: WordType::Quality,
            },
            Word {
                kana: "ko",
                english: vec!["soft", "clay", "semisolid", "paste"],
                wtype: WordType::Quality,
            },
            Word {
                kana: "namako",
                english: vec!["spice", "extra", "additional", "flavor"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "oko",
                english: vec!["eye", "vision"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "melome",
                english: vec!["woman", "female", "feminine"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "mije",
                english: vec!["man", "male", "masculine"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "kasi",
                english: vec!["plant", "leaf", "herb", "grow"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "sike",
                english: vec!["circle", "round", "ball", "cycle"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "len",
                english: vec!["cloth", "clothing", "fabric", "cover"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "unpa",
                english: vec!["sexual", "marriage", "intimate"],
                wtype: WordType::Action,
            },
            Word {
                kana: "pakala",
                english: vec!["break", "damage", "destroy", "mistake"],
                wtype: WordType::Action,
            },
            Word {
                kana: "selo",
                english: vec!["outer", "skin", "surface", "boundary"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "leko",
                english: vec!["square", "block", "corner"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "lanpan",
                english: vec!["take", "get", "receive", "grab"],
                wtype: WordType::Action,
            },
            Word {
                kana: "wan",
                english: vec!["one", "unique", "unite"],
                wtype: WordType::Number,
            },
            Word {
                kana: "tu",
                english: vec!["two", "split", "divide"],
                wtype: WordType::Number,
            },
            Word {
                kana: "mute",
                english: vec!["many", "several", "much", "quantity"],
                wtype: WordType::Number,
            },
            Word {
                kana: "nanpa",
                english: vec!["number", "order", "th"],
                wtype: WordType::Number,
            },
            Word {
                kana: "lawa",
                english: vec!["head", "lead", "control", "main", "ruler"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "kipisi",
                english: vec!["split", "cut", "divide", "slice"],
                wtype: WordType::Action,
            },
            Word {
                kana: "monsuta",
                english: vec!["fear", "monster", "danger", "scary"],
                wtype: WordType::Quality,
            },
            Word {
                kana: "tonsili",
                english: vec!["health", "wellness", "safe"],
                wtype: WordType::Quality,
            },
            Word {
                kana: "meso",
                english: vec!["medium", "average", "middle"],
                wtype: WordType::Quality,
            },
            Word {
                kana: "jami",
                english: vec!["tasty", "delicious", "yummy"],
                wtype: WordType::Quality,
            },
            Word {
                kana: "suwi",
                english: vec!["sweet", "cute", "candy", "sugar"],
                wtype: WordType::Quality,
            },
            Word {
                kana: "wile",
                english: vec!["want", "need", "must", "should"],
                wtype: WordType::Action,
            },
            Word {
                kana: "kijetesantakalu",
                english: vec!["raccoon", "ferret", "mustelid"],
                wtype: WordType::Entity,
            },
            Word {
                kana: "yu",
                english: vec!["hello", "greeting", "hi"],
                wtype: WordType::Particle,
            },
            Word {
                kana: "poka",
                english: vec!["friend", "companion", "together"],
                wtype: WordType::Entity,
            },
        ];

        let mut en_to_kana = HashMap::new();
        let mut kana_to_en = HashMap::new();
        let mut word_types = HashMap::new();

        for word in &words {
            word_types.insert(word.kana.to_string(), word.wtype);
            let meanings: Vec<&str> = word.english.clone();
            kana_to_en.insert(word.kana.to_string(), meanings.clone());

            for meaning in &word.english {
                if !meaning.starts_with('[') {
                    en_to_kana.insert(meaning.to_lowercase(), word.kana);
                }
            }
        }

        Dictionary {
            en_to_kana,
            kana_to_en,
            word_types,
        }
    }

    pub fn translate_en_word(&self, word: &str) -> Option<&str> {
        let lower = word.to_lowercase();
        self.en_to_kana.get(&lower).copied()
    }

    pub fn translate_kana_word(&self, word: &str) -> Option<&Vec<&str>> {
        self.kana_to_en.get(word)
    }

    pub fn get_word_type(&self, word: &str) -> Option<WordType> {
        self.word_types.get(word).copied()
    }
}
