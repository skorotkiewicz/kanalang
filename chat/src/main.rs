use colored::Colorize;
use kanalang::Translator;
use reqwest::Client;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

const SYSTEM_PROMPT: &str = r#"You are a helpful assistant that responds in Kanalang, a simple constructed language based on Toki Pona.

GRAMMAR RULES:
- Word order: Subject + li + Verb + e + Object
- "li" separates subject from verb (omit if subject is "mi" or "sina")
- "e" marks the direct object
- "se" at the start makes a question
- "ala" after a word negates it
- "pi" groups modifiers together

CORE VOCABULARY:
Pronouns: mi (I/me), sina (you), ona (he/she/it/they)
Entities: jan (person), tomo (house), ma (land/place), ilo (tool), kala (fish), kasi (plant), telo (water), suno (sun/day), mun (moon), kon (air), seli (fire), lete (cold)
Actions: toki (speak), wile (want), sona (know), lukin (see), kute (hear), moku (eat), lape (sleep), pali (do/make), tawa (go), kama (come), jo (have), pana (give), olin (love), ken (can)
Qualities: pona (good), ike (bad), suli (big), lili (small), wawa (strong), mute (many/very), sin (new), pini (done/finished)
Particles: li (subject marker), e (object marker), la (context marker), en (and), anu (or), ala (no/not), kin (also)

EXAMPLES:
- "mi wile e moku" = I want food
- "sina pona" = You are good
- "mi moku" = I eat
- "se sina lon" = Are you here?
- "ona li toki e ijo" = They say something
- "mi wile ala" = I don't want

IMPORTANT: Always respond in Kanalang only. Use simple sentences. If you don't know a word, use [word] brackets."#;

fn print_usage() {
    println!(
        "{} {}",
        "kanalang-chat - Chat with LLM in Kanalang".white().bold(),
        env!("CARGO_PKG_VERSION").dimmed()
    );
    println!();
    println!("{}", "USAGE:".white().underline());
    println!(
        "  {}",
        "$ chat --endpoint http://localhost:8080/v1 --model default --api-key 123"
            .bright_green()
    );
    println!();
    println!("{}", "OPTIONS:".white().underline());
    println!(
        "  {}  {}",
        "--endpoint <url>".green(),
        "OpenAI-compatible API endpoint".white()
    );
    println!(
        "  {}  {}",
        "--model <name>".green(),
        "Model name to use".white()
    );
    println!(
        "  {}  {}",
        "--api-key <key>".green(),
        "API key for authentication".white()
    );
    println!();
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && (args[1] == "-h" || args[1] == "--help" || args[1] == "help") {
        print_usage();
        return;
    }

    let mut endpoint = String::new();
    let mut model = String::new();
    let mut api_key = String::new();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--endpoint" if i + 1 < args.len() => {
                endpoint = args[i + 1].clone();
                i += 2;
            }
            "--model" if i + 1 < args.len() => {
                model = args[i + 1].clone();
                i += 2;
            }
            "--api-key" if i + 1 < args.len() => {
                api_key = args[i + 1].clone();
                i += 2;
            }
            _ => {
                eprintln!("Unknown option: {}", args[i]);
                print_usage();
                return;
            }
        }
    }

    if endpoint.is_empty() || model.is_empty() || api_key.is_empty() {
        eprintln!(
            "{} --endpoint, --model, and --api-key are required",
            "error:".red()
        );
        print_usage();
        return;
    }

    let translator = Translator::new();
    let client = Client::new();
    let mut messages: Vec<Message> = vec![Message {
        role: "system".to_string(),
        content: SYSTEM_PROMPT.to_string(),
    }];

    let mut rl = match DefaultEditor::new() {
        Ok(rl) => rl,
        Err(e) => {
            eprintln!("Failed to initialize readline: {}", e);
            return;
        }
    };

    println!("{}", "Kanalang Chat - Type 'quit' to exit".cyan().bold());
    println!(
        "{}",
        "Your messages will be translated to kanalang before sending.".dimmed()
    );
    println!(
        "{}",
        "LLM responses will be translated back to English.".dimmed()
    );
    println!();

    loop {
        let readline = rl.readline("you> ");
        match readline {
            Ok(line) => {
                let input = line.trim();
                if input.is_empty() {
                    continue;
                }
                if input == "quit" || input == "exit" {
                    println!("{}", "Goodbye!".dimmed());
                    break;
                }

                let _ = rl.add_history_entry(input);

                let kana_input = translator.english_to_kana(input);
                println!("{} {}", "[kanalang]".dimmed(), kana_input.dimmed());

                messages.push(Message {
                    role: "user".to_string(),
                    content: kana_input.clone(),
                });

                let request = ChatRequest {
                    model: model.clone(),
                    messages: messages.clone(),
                };

                let response = client
                    .post(format!("{}/chat/completions", endpoint))
                    .header("Authorization", format!("Bearer {}", api_key))
                    .header("Content-Type", "application/json")
                    .json(&request)
                    .send()
                    .await;

                match response {
                    Ok(resp) => {
                        if resp.status().is_success() {
                            match resp.json::<ChatResponse>().await {
                                Ok(chat_resp) => {
                                    if let Some(choice) = chat_resp.choices.first() {
                                        let kana_response = &choice.message.content;
                                        println!(
                                            "{} {}",
                                            "[kanalang]".dimmed(),
                                            kana_response.dimmed()
                                        );

                                        let english_response =
                                            translator.kana_to_english(kana_response);
                                        println!(
                                            "{} {}",
                                            "llm>".green().bold(),
                                            english_response.white()
                                        );

                                        messages.push(choice.message.clone());
                                    }
                                }
                                Err(e) => {
                                    eprintln!("{} Failed to parse response: {}", "error:".red(), e);
                                }
                            }
                        } else {
                            let status = resp.status();
                            let text = resp.text().await.unwrap_or_default();
                            eprintln!("{} API error ({}): {}", "error:".red(), status, text);
                        }
                    }
                    Err(e) => {
                        eprintln!("{} Request failed: {}", "error:".red(), e);
                    }
                }
                println!();
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("^D");
                break;
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }
}
