//! Module for calculating indentation levels in source code.

/// Calculate indentation levels with a custom tab width.
///
/// Returns a vector where each element represents the indentation level
/// (in spaces) of a line that contains non-whitespace characters.
/// Lines that contain only whitespace receive the previous line's indentation.
///
/// # Examples
///
/// ```
/// use blink_pairs::indent::indent_levels;
///
/// let src = ["fn main() {", "\tprintln!(\"hello\");", "}"];
/// let indents = indent_levels(&src, 8);
/// assert_eq!(indents, vec![0, 4, 0]);
/// ```
pub fn indent_levels(lines: &[&str], tab_width: u8) -> Vec<u8> {
    let mut last_indent = None;
    let mut indents = Vec::with_capacity(lines.len());
    'outer: for line in lines {
        let mut indent: u8 = 0;
        for c in line.as_bytes() {
            match c {
                b' ' => indent = indent.saturating_add(1),
                b'\t' => indent = indent.saturating_add(tab_width),
                _ => {
                    indents.push(indent);
                    last_indent = Some(indent);
                    continue 'outer;
                }
            }
        }
        // this line is entirely whitespace, so use the previous line's indentation.
        indents.push(*last_indent.get_or_insert(indent));
    }
    indents
}

#[cfg(test)]
mod tests {
    use super::indent_levels;

    #[test]
    fn test_basic_indentation() {
        let src = ["if foo() {", "    bar();", "}"];
        let result = indent_levels(&src, 4);
        assert_eq!(result, vec![0, 4, 0]);
    }

    #[test]
    fn test_mixed_tabs_and_spaces() {
        let src = [
            "if foo() {",
            "    if bar {",
            "\t\tprintln!(\"hello world\");",
            "    }",
            "}",
        ];
        let result = indent_levels(&src, 4);
        assert_eq!(result, vec![0, 4, 8, 4, 0]);
    }

    #[test]
    fn test_empty_lines() {
        let src = ["line1", "", "    line3", "", "        line5"];
        let result = indent_levels(&src, 4);
        assert_eq!(result, vec![0, 0, 4, 4, 8]);
    }

    #[test]
    fn test_only_empty_lines() {
        let src = ["", "", "", ""];
        let result = indent_levels(&src, 4);
        assert_eq!(result, vec![0, 0, 0, 0]);
    }

    #[test]
    fn test_only_whitespace_lines() {
        let src = ["    ", "", "", ""];
        let result = indent_levels(&src, 4);
        assert_eq!(result, vec![4, 4, 4, 4]);
    }

    #[test]
    fn test_all_whitespace_lines() {
        let src = ["line1", "    ", "\t", "    line4"];
        let result = indent_levels(&src, 4);
        assert_eq!(result, vec![0, 0, 0, 4]);
    }

    #[test]
    fn test_different_tab_width() {
        let src = ["\tindented", "\t\tdouble"];
        let result = indent_levels(&src, 8);
        assert_eq!(result, vec![8, 16]);
    }

    #[test]
    fn test_no_trailing_newline() {
        let src = ["line1", "    line2"];
        let result = indent_levels(&src, 4);
        assert_eq!(result, vec![0, 4]);
    }

    #[test]
    fn test_only_whitespace() {
        let src = ["    "];
        let result = indent_levels(&src, 4);
        assert_eq!(result, vec![4]);
    }

    #[test]
    fn test_empty_string() {
        let src = [""];
        let result = indent_levels(&src, 4);
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn test_single_line_no_indentation() {
        let src = ["hello world"];
        let result = indent_levels(&src, 4);
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn test_large_input() {
        let src: [&str; 2] = [&"a".repeat(40), &(" ".repeat(40) + "b")];
        let result = indent_levels(&src, 4);
        assert_eq!(result, vec![0, 40]);
    }

    #[test]
    fn test_windows_line_endings() {
        // The current implementation treats \r as a non-whitespace character
        let src = ["line1\r", "    line2\r", ""];
        let result = indent_levels(&src, 4);
        assert_eq!(result, vec![0, 4, 4]);
    }
}
