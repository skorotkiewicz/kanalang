use colored::Colorize;
use kanalang::Translator;
use std::env;
use std::io::{self, BufRead, Write};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn print_usage() {
    println!(
        "{} {}",
        "kana - type-safe natural language translator"
            .white()
            .bold(),
        VERSION.dimmed()
    );
    println!();
    println!("{}", "USAGE:".white().underline());
    println!("  {}", "$ kanalang to \"i want food\"".bright_green());
    println!("  {}", "mi wile e moku".bright_black());
    println!();
    println!("  {}", "$ kanalang from \"mi toki pona\"".bright_green());
    println!("  {}", "I speak good.".bright_black());
    println!();
    println!("  {}", "$ echo \"i love you\" | kanalang to".bright_green());
    println!("  {}", "mi olin e sina".bright_black());
    println!();
    println!("{}", "COMMANDS:".white().underline());
    println!(
        "  {}  {}",
        "to, en2k".green().bold(),
        "Translate English → Kana".white()
    );
    println!(
        "  {}  {}",
        "from, k2en".green().bold(),
        "Translate Kana → English".white()
    );
    println!(
        "  {}  {}",
        "(none)".dimmed(),
        "Auto-detect language".white()
    );
    println!();
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
