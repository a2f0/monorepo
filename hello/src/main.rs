fn get_pid_message() -> String {
    format!("Hello. My PID is {}", std::process::id())
}

fn main() {
    println!("{}", get_pid_message());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }

    #[test]
    fn test_get_pid_message() {
        let message = get_pid_message();
        assert!(message.starts_with("Hello. My PID is "));
        assert!(message.len() > 14); // Basic length check to ensure PID is included
    }
}
