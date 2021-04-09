mod utils {}

pub fn print_difficulty(difficulty: &str, mut writer: impl std::io::Write) {
    writeln!(writer, "Playing on: {}", difficulty);
}
