# Kanalang

Type-safe natural language translator. No LLM. Inspired by Toki Pona.

## Install

```bash
# Build both binaries
cargo build --release

# Build only kanalang (minimal)
cargo build --release -p kanalang

# Build only chat
cargo build --release -p chat
```

## Usage

### kanalang - Translator

```bash
# English → Kana
./target/release/kanalang to "i want food"
# mi wile e moku

# Kana → English
./target/release/kanalang from "mi olin e sina"
# I love you.

# Pipe
echo "i love you" | ./target/release/kanalang to
# mi olin e sina
```

### chat - LLM Chat Interface

Chat with any OpenAI-compatible LLM. Messages are translated to kanalang before sending and translated back to English.

```bash
./target/release/chat --endpoint http://localhost:8080/v1 --model default --api-key 123
```

Example session:
```
you> hello
[kanalang] yu
llm> hello.

you> i want food
[kanalang] mi wile e moku
[kanalang] mi pana e moku tawa sina.
llm> I give food to you.
```

## Grammar

```
Subject li Verb e Object
```

- `li` - verb marker
- `e` - object marker
- `se` - question
- `ala` - negation

## Examples

| English | Kana |
|---------|------|
| hello | yu |
| i love you | mi olin e sina |
| i see you | mi lukin e sina |
| person eat food | jan li moku e pan |

## Philosophy

~120 simple words. No secrets can hide. Transparent communication.
