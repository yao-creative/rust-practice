pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_with_bob() {
        let result = greet("Bob");
        assert_eq!(result, "Hello, Bob!");
    }
}
