//! Module for calculating indentation levels in source code.

use std::simd::{cmp::SimdPartialEq, LaneCount, Simd, SupportedLaneCount};

/// Calculate indentation levels with a custom tab width.
///
/// Returns a vector where each element represents the indentation level
/// (in spaces) of a line that contains non-whitespace characters.
/// Lines that are empty or contain only whitespace are not included.
///
/// # Examples
///
/// ```
/// use blink_pairs::indent::indent_levels;
///
/// let src = "fn main() {\n\tprintln!(\"hello\");\n}";
/// let indents = indent_levels(src, 8);
/// assert_eq!(indents, vec![0, 8, 0]);
/// ```
pub fn indent_levels<const N: usize>(src: &str, tab_width: u8) -> Vec<u8>
where
    LaneCount<N>: SupportedLaneCount,
{
    let mut in_indentation = true;
    let mut indentation = Vec::new();
    let mut current_indentation: u8 = 0;

    let (chunks, remainder) = src.as_bytes().as_chunks::<N>();

    for &chunk in chunks.iter() {
        let chunk = Simd::from_array(chunk);
        let newlines = chunk.simd_eq(Simd::splat(b'\n'));

        if !in_indentation && !newlines.any() {
            continue;
        }

        let spaces = chunk.simd_eq(Simd::splat(b' '));
        let tabs = chunk.simd_eq(Simd::splat(b'\t'));
        let whitespace = spaces.select(Simd::splat(1), Simd::splat(0))
            + tabs.select(Simd::splat(tab_width), Simd::splat(0));
        let whitespace = whitespace.to_array();

        if in_indentation {
            for &c in &whitespace {
                if c > 0 {
                    current_indentation = current_indentation.saturating_add(c);
                } else {
                    in_indentation = false;
                    break;
                }
            }
        }

        let mut newlines = newlines.to_bitmask();
        loop {
            if newlines == 0 {
                // there are no more newlines in this chunk
                break;
            }

            // push the previous line's indentation
            in_indentation = true;
            indentation.push(current_indentation);
            current_indentation = 0;

            // find the index of the next newline in the chunk (the lowest-set bit)
            let idx = newlines.trailing_zeros();
            // clear the lowest-set bit
            newlines &= newlines - 1;

            // idx + 1 is the index of the character after the newline. This indexing is safe
            // because arr[arr.len()..] does not panic (and iterates over nothing).
            for &c in &whitespace[(idx + 1) as usize..] {
                if c > 0 {
                    current_indentation = current_indentation.saturating_add(c);
                } else {
                    in_indentation = false;
                    break;
                }
            }
        }
    }

    for &c in remainder {
        if c == b'\n' {
            in_indentation = true;
            indentation.push(current_indentation);
            current_indentation = 0;
        } else if in_indentation {
            match c {
                b' ' => current_indentation = current_indentation.saturating_add(1),
                b'\t' => current_indentation = current_indentation.saturating_add(tab_width),
                _ => {
                    in_indentation = false;
                }
            }
        }
    }

    indentation.push(current_indentation);

    indentation
}

#[cfg(test)]
mod tests {
    use super::indent_levels;

    #[test]
    fn test_basic_indentation() {
        let src = "if foo() {\n    bar();\n}";
        let result = indent_levels::<32>(src, 4);
        assert_eq!(result, vec![0, 4, 0]);
    }

    #[test]
    fn test_mixed_tabs_and_spaces() {
        let src = "if foo() {\n    if bar {\n\t\tprintln!(\"hello world\");\n    }\n}";
        let result = indent_levels::<32>(src, 4);
        assert_eq!(result, vec![0, 4, 8, 4, 0]);
    }

    #[test]
    fn test_empty_lines() {
        let src = "line1\n\n    line3\n\n        line5";
        let result = indent_levels::<32>(src, 4);
        assert_eq!(result, vec![0, 0, 4, 0, 8]);
    }

    #[test]
    fn test_only_empty_lines() {
        let src = "\n\n\n";
        let result = indent_levels::<32>(src, 4);
        assert_eq!(result, vec![0, 0, 0, 0]);
    }

    #[test]
    fn test_all_whitespace_lines() {
        let src = "line1\n    \n\t\n    line4";
        let result = indent_levels::<32>(src, 4);
        assert_eq!(result, vec![0, 4, 4, 4]);
    }

    #[test]
    fn test_different_tab_width() {
        let src = "\tindented\n\t\tdouble";
        let result = indent_levels::<32>(src, 8);
        assert_eq!(result, vec![8, 16]);
    }

    #[test]
    fn test_no_trailing_newline() {
        let src = "line1\n    line2";
        let result = indent_levels::<32>(src, 4);
        assert_eq!(result, vec![0, 4]);
    }

    #[test]
    fn test_only_whitespace() {
        let src = "    ";
        let result = indent_levels::<32>(src, 4);
        assert_eq!(result, vec![4]);
    }

    #[test]
    fn test_empty_string() {
        let src = "";
        let result = indent_levels::<32>(src, 4);
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn test_single_line_no_indentation() {
        let src = "hello world";
        let result = indent_levels::<32>(src, 4);
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn test_large_input() {
        // Test with input larger than SIMD chunk size (32 bytes)
        let src = "a".repeat(40) + "\n" + &" ".repeat(40) + "b";
        let result = indent_levels::<32>(&src, 4);
        assert_eq!(result, vec![0, 40]);
    }

    #[test]
    fn test_windows_line_endings() {
        // The current implementation treats \r as a non-whitespace character
        let src = "line1\r\n    line2\r\n";
        let result = indent_levels::<32>(src, 4);
        assert_eq!(result, vec![0, 4, 0]);
    }
}
