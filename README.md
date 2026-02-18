# Kana

Type-safe natural language translator. No LLM. Inspired by Toki Pona.

## Usage

```bash
# Build
cargo build --release

# English → Kana
echo "i want food" | ./target/release/kanalang to
# mi wile e pan

# Kana → English
echo "mi olin e sina" | ./target/release/kanalang from
# I love you.

# Auto-detect
./target/release/kanalang "mi toki pona"
# I speak good.
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
