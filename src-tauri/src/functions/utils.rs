// Credits: https://stackoverflow.com/questions/38406793/why-is-capitalizing-the-first-letter-of-a-string-so-convoluted-in-rust
pub fn uppercase(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
