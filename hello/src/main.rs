fn main() {
    println!("Hello. My PID is {}", std::process::id());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_main() {
        super::main();
    }
}
