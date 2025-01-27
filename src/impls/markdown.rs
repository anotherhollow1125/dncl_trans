pub fn extract_rust_codes(markdown: &str) -> Vec<String> {
    let mut rust_code_blocks = Vec::new();
    let mut in_rust_block = false;
    let mut current_block = String::new();

    for line in markdown.lines() {
        if line.trim_start().starts_with("```rust") {
            in_rust_block = true;
            current_block.clear();
        } else if line.trim_start().starts_with("```") && in_rust_block {
            in_rust_block = false;
            rust_code_blocks.push(current_block.clone());
        } else if in_rust_block {
            current_block.push_str(line);
            current_block.push('\n');
        }
    }

    rust_code_blocks
}

#[cfg(test)]
mod tests {
    use super::extract_rust_codes;

    #[test]
    fn test_extract_rust_codes() {
        let input = r#"
aaaa        
        
```rust
fn main() {
    println!("Hello, world!");
}
```

bbbbb

```rust
fn hoge() {
    println!("hoge");
}
```

cccccc"#;
        let expected = vec![
            r#"fn main() {
    println!("Hello, world!");
}
"#
            .to_string(),
            r#"fn hoge() {
    println!("hoge");
}
"#
            .to_string(),
        ];

        let res = extract_rust_codes(input);
        assert_eq!(res, expected);
    }
}
