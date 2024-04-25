

use std::io::{self, BufRead};


pub fn grep<R>(target: &str, reader: R) -> io::Result<()>
    where R: BufRead
{
    for line_result in reader.lines() {
        let line = line_result?;
        if line.contains(target) {
            println!("{}", line);
        }
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_grep_single_line() {
        let input = "hello world";
        let target = "hello";
        let cursor = Cursor::new(input);
        assert!(grep(target, cursor).is_ok());
    }

    #[test]
    fn test_grep_multiple_lines() {
        let input = "hello world\nfoo bar\nbaz";
        let target = "foo";
        let cursor = Cursor::new(input);
        assert!(grep(target, cursor).is_ok());
    }

    #[test]
    fn test_grep_empty_input() {
        let input = "";
        let target = "foo";
        let cursor = Cursor::new(input);
        assert!(grep(target, cursor).is_ok());
    }

    #[test]
    fn test_grep_empty_target() {
        let input = "hello world";
        let target = "";
        let cursor = Cursor::new(input);
        assert!(grep(target, cursor).is_ok());
    }

    #[test]
    fn test_grep_target_not_found() {
        let input = "hello world";
        let target = "foo";
        let cursor = Cursor::new(input);
        assert!(grep(target, cursor).is_ok());
    }

    #[test]
    fn test_grep_target_found_twice() {
        let input = "hello world\nhello universe";
        let target = "hello";
        let cursor = Cursor::new(input);
        assert!(grep(target, cursor).is_ok());
    }

    #[test]
    fn test_grep_long_input() {
        let input = "a\nb\nc\nd\ne\n".repeat(100);
        let target = "c";
        let cursor = Cursor::new(input);
        assert!(grep(target, cursor).is_ok());
    }

    #[test]
    fn test_grep_long_target() {
        let input = "hello world";
        let target = "hello worldhello worldhello worldhello worldhello world";
        let cursor = Cursor::new(input);
        assert!(grep(target, cursor).is_ok());
    }

    #[test]
    fn test_grep_with_special_characters() {
        let input = "line1: special characters!@#$%\nline2: more_special_characters^&*()\n";
        let target = "!@#$%";
        let cursor = Cursor::new(input);
        assert!(grep(target, cursor).is_ok());
    }

    #[test]
    fn test_grep_with_unicode() {
        let input = "こんにちは\n안녕하세요\n你好\n";
        let target = "안녕하세요";
        let cursor = Cursor::new(input);
        assert!(grep(target, cursor).is_ok());
    }
}

