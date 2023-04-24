# dfa-regex

Simple DFA Regex engine wirrten in Rust.

## Example

```rust
#[test]
fn case01() {
    let regex = Regex::new(r"(p(erl|ython|hp)|ruby)").unwrap();
    assert!(regex.matches("python"));
    assert!(regex.matches("ruby"));
    assert!(regex.matches("perl"));
    assert!(!regex.matches("ruby2"));
    assert!(!regex.matches("java"));
    assert!(!regex.matches("VB"));
}
```

For more examples, please see [tests/matches.rs](tests/matches.rs)

## Supported Features

You can use Unicode characters such as `a`, `A`, `あ`.

- `\`: Escape character. e.g. `\(` `\+`
- `|`: OR operator. e.g. `a|b`
- `*`: Repeat more than 0. e.g. `a*`
- `+`: Repeat more than 1. e.g. `a+`
- `(` and `)`: e.g. `a(b|c)*`
