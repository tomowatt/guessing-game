mod tests {}

use crate::utils::print_difficulty;

#[test]
fn test_print_difficulty_empty() {
    let mut result = Vec::new();
    print_difficulty("", &mut result);
    assert_eq!(result, b"Playing on: \n");
}

#[test]
fn test_print_difficulty() {
    let mut result = Vec::new();
    print_difficulty("easy", &mut result);
    assert_eq!(result, b"Playing on: easy\n");
}
