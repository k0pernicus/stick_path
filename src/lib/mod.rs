pub mod solver;

/// replace_chars replaces all '--' strings with '> <' in order to perform easy parsing
pub fn replace_chars(s: &str) -> String {
    s.replace("--", "> <").replace("|>", ">").replace("<|", "<")
}

#[cfg(test)]
mod test {
    use super::replace_chars;
    #[test]
    fn test_replacement() {
        let simple_replacement = "|--|";
        assert_eq!(replace_chars(simple_replacement), "> <");
        let left_replacement = "| ||>|<|";
        let right_replacement = "| |>|<";
        assert_eq!(replace_chars(left_replacement), right_replacement);
    }
    #[test]
    fn test_no_replacement() {
        let no_replacement = "|-|-|< | >";
        assert_eq!(replace_chars(no_replacement), no_replacement);
    }
}
