pub fn get_pid_message() -> String {
    format!("Hello. My PID is {}", std::process::id())
}

pub fn main() {
    println!("{}", get_pid_message());
}
