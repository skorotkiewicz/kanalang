mod dict;
mod parser;
mod translator;

use std::env;
use std::io::{self, BufRead, Write};
use translator::Translator;

fn print_usage() {
    eprintln!("kana - type-safe natural language translator");
    eprintln!();
    eprintln!("USAGE:");
    eprintln!("  echo \"from english to natural\" | ./bluep to");
    eprintln!("  echo \"mi toki pona\" | ./bluep from");
    eprintln!("  ./bluep to \"hello world\"");
    eprintln!("  ./bluep from \"mi wile moku\"");
    eprintln!();
    eprintln!("COMMANDS:");
    eprintln!("  to, en2k    Translate English → Kana");
    eprintln!("  from, k2en  Translate Kana → English");
    eprintln!("  (none)      Auto-detect language");
    eprintln!();
    eprintln!("EXAMPLES:");
    eprintln!("  $ ./bluep to \"i want food\"");
    eprintln!("  mi wile e moku");
    eprintln!();
    eprintln!("  $ ./bluep from \"mi toki pona\"");
    eprintln!("  I speak good.");
    eprintln!();
    eprintln!("  $ echo \"i love you\" | ./bluep to");
    eprintln!("  mi olin e sina");
    eprintln!();
    eprintln!("KANA LANGUAGE:");
    eprintln!("  • ~120 simple words");
    eprintln!("  • Type-safe grammar with particles (li, e, pi)");
    eprintln!("  • SVO word order");
    eprintln!("  • Transparent - no secrets can hide");
    eprintln!("  • Inspired by Toki Pona");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && (args[1] == "-h" || args[1] == "--help" || args[1] == "help") {
        print_usage();
        return;
    }

    let translator = Translator::new();

    let direction = if args.len() > 1 {
        match args[1].as_str() {
            "to" | "en2k" | "en-kana" => "to",
            "from" | "k2en" | "kana-en" => "from",
            _ => "auto",
        }
    } else {
        "auto"
    };

    if args.len() > 2 {
        let input = args[2..].join(" ");
        let output = translator.translate(&input, direction);
        println!("{}", output);
        return;
    }

    if args.len() == 2 && direction != "auto" {
        let stdin = io::stdin();
        let mut output = String::new();

        for line in stdin.lock().lines() {
            match line {
                Ok(text) => {
                    output.push_str(&translator.translate(&text, direction));
                    output.push('\n');
                }
                Err(_) => break,
            }
        }

        let stdout = io::stdout();
        let mut handle = stdout.lock();
        let _ = handle.write_all(output.as_bytes());
        return;
    }

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(text) => {
                if !text.trim().is_empty() {
                    let output = translator.translate(&text, direction);
                    println!("{}", output);
                }
            }
            Err(_) => break,
        }
    }
}
