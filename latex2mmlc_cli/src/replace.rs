use std::fmt;

use memchr::memmem::Finder;

use latex2mmlc::{Display, LatexError};

use crate::html_entities::replace_html_entities;

#[derive(Debug)]
pub enum ConversionError<'source> {
    UnclosedDelimiter(usize),
    NestedDelimiters(usize),
    MismatchedDelimiters(usize, usize),
    LatexError(LatexError<'source>, &'source str),
}
impl fmt::Display for ConversionError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConversionError::UnclosedDelimiter(idx) => write!(f, "Unclosed delimiter at {idx}"),
            ConversionError::NestedDelimiters(idx) => {
                write!(f, "Nested delimiters are not allowed (at {idx})")
            }
            ConversionError::MismatchedDelimiters(open, close) => {
                write!(f, "Mismatched delimiters at {open} and {close}")
            }
            ConversionError::LatexError(e, input) => {
                write!(f, "Error at {} in '{}':\n{}", e.0, input, e)
            }
        }
    }
}
impl std::error::Error for ConversionError<'_> {}

pub struct Replacer<'config> {
    opening_finders: (Finder<'config>, Finder<'config>),
    closing_finders: (Finder<'config>, Finder<'config>),
    opening_lengths: (usize, usize),
    closing_lengths: (usize, usize),
    closing_identical: bool,
    entity_buffer: String,
}

impl<'config> Replacer<'config> {
    pub fn new(
        inline_delim: (&'config str, &'config str),
        block_delim: (&'config str, &'config str),
    ) -> Self {
        let inline_opening = Finder::new(inline_delim.0);
        let inline_closing = Finder::new(inline_delim.1);
        let block_opening = Finder::new(block_delim.0);
        let block_closing = Finder::new(block_delim.1);

        Self {
            opening_finders: (inline_opening, block_opening),
            closing_finders: (inline_closing, block_closing),
            opening_lengths: (inline_delim.0.len(), block_delim.0.len()),
            closing_lengths: (inline_delim.1.len(), block_delim.1.len()),
            closing_identical: inline_delim.1 == block_delim.1,
            entity_buffer: String::new(),
        }
    }

    /// Replaces the content of inline and block math delimiters in a LaTeX string.
    ///
    /// Any kind of nesting of delimiters is not allowed.
    #[inline]
    pub(crate) fn replace<'source, 'buf, F>(
        &'buf mut self,
        input: &'source str,
        f: F,
    ) -> Result<String, ConversionError<'buf>>
    where
        F: for<'a> Fn(&mut String, &'a str, Display) -> Result<(), LatexError<'a>>,
        'source: 'buf,
    {
        let mut output = String::with_capacity(input.len());
        let mut current_pos = 0;

        // while current_pos < input.len() {
        let error_typ = loop {
            if current_pos >= input.len() {
                break None;
            }
            let remaining = &input[current_pos..];

            // Find the next occurrence of any opening delimiter
            let opening = self.find_next_delimiter(remaining, true);

            let Some((open_typ, idx)) = opening else {
                // No more opening delimiters found
                output.push_str(remaining);
                break None;
            };

            let opening_delim_len = match open_typ {
                Display::Inline => self.opening_lengths.0,
                Display::Block => self.opening_lengths.1,
            };

            let open_pos = current_pos + idx;
            // Append everything before the opening delimiter
            output.push_str(&input[current_pos..open_pos]);
            // Skip the opening delimiter itself
            let start = open_pos + opening_delim_len;
            let remaining = &input[start..];

            // Find the next occurrence of any closing delimiter
            let closing = self.find_next_delimiter(remaining, false);

            let Some((close_typ, idx)) = closing else {
                // No closing delimiter found
                return Err(ConversionError::UnclosedDelimiter(open_pos));
            };

            let closing_delim_len = match close_typ {
                Display::Inline => self.closing_lengths.0,
                Display::Block => self.closing_lengths.1,
            };

            if !self.closing_identical && open_typ != close_typ {
                // Mismatch of opening and closing delimiter
                return Err(ConversionError::MismatchedDelimiters(open_pos, start + idx));
            }

            let end = start + idx;
            // Get the content between delimiters
            let content = &input[start..end];
            // Check whether any *opening* delimiters are present in the content
            if let Some((_, idx)) = self.find_next_delimiter(content, true) {
                return Err(ConversionError::NestedDelimiters(start + idx));
            }
            // Replace HTML entities
            replace_html_entities(&mut self.entity_buffer, content);
            // Convert the content
            let result = f(&mut output, self.entity_buffer.as_str(), open_typ);
            match result {
                Ok(_) => {}
                Err(_) => {
                    // We know a problem occurred in the slice `replaced`
                    break Some(open_typ);
                }
            }
            // Update current position
            current_pos = end + closing_delim_len;
        };

        if let Some(open_typ) = error_typ {
            // Do the conversion again so we can get the error.
            let result = f(&mut output, self.entity_buffer.as_str(), open_typ).unwrap_err();
            return Err(ConversionError::LatexError(
                result,
                self.entity_buffer.as_str(),
            ));
        }

        Ok(output)
    }

    /// Finds the next occurrence of either an inline or block delimiter.
    fn find_next_delimiter(&self, input: &str, opening: bool) -> Option<(Display, usize)> {
        let (inline_finder, block_finder) = if opening {
            (&self.opening_finders.0, &self.opening_finders.1)
        } else {
            (&self.closing_finders.0, &self.closing_finders.1)
        };

        let inline_pos = inline_finder.find(input.as_bytes());
        let block_pos = block_finder.find(input.as_bytes());

        match (inline_pos, block_pos) {
            // If we have i == d, Display has priority
            (Some(i), Some(d)) if i < d => Some((Display::Inline, i)),
            (_, Some(d)) => Some((Display::Block, d)),
            (Some(i), None) => Some((Display::Inline, i)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Write;

    /// Mock convert function for testing
    fn mock_convert<'source>(
        buf: &mut String,
        content: &'source str,
        typ: Display,
    ) -> Result<(), LatexError<'source>> {
        match typ {
            Display::Inline => write!(buf, "[T1:{}]", content).unwrap(),
            Display::Block => write!(buf, "[T2:{}]", content).unwrap(),
        };
        Ok(())
    }

    fn replace(
        input: &'static str,
        inline_delim: (&'static str, &'static str),
        block_delim: (&'static str, &'static str),
    ) -> Result<String, ConversionError<'static>> {
        let mut replacer = Replacer::new(inline_delim, block_delim);
        match replacer.replace(input, |buf, content, typ| mock_convert(buf, content, typ)) {
            Ok(s) => Ok(s),
            Err(e) => match e {
                // The following is needed to do a kind of "lifetime laundering".
                ConversionError::MismatchedDelimiters(a, b) => {
                    Err(ConversionError::MismatchedDelimiters(a, b))
                }
                ConversionError::NestedDelimiters(a) => Err(ConversionError::NestedDelimiters(a)),
                ConversionError::UnclosedDelimiter(a) => Err(ConversionError::UnclosedDelimiter(a)),
                ConversionError::LatexError(_, _) => unreachable!(),
            },
        }
    }

    #[test]
    fn test_basic_replacement() {
        let input = "Hello $world$ and $$universe$$";
        let result = replace(input, ("$", "$"), ("$$", "$$")).unwrap();
        assert_eq!(result, "Hello [T1:world] and [T2:universe]");
    }

    #[test]
    fn test_nested_delimiters() {
        let input = "Nested $$outer $inner$ delimiter$$";
        let result = replace(input, ("$", "$"), ("$$", "$$")).unwrap_err();
        println!("{}", result);
        assert!(matches!(
            result,
            ConversionError::MismatchedDelimiters(7, 15)
        ));
    }

    #[test]
    fn test_nested_delimiters2() {
        let input = "Nested $outer $$inner$$ delimiter$";
        let result = replace(input, ("$", "$"), ("$$", "$$")).unwrap_err();
        println!("{}", result);
        assert!(matches!(
            result,
            ConversionError::MismatchedDelimiters(7, 14)
        ));
    }

    #[test]
    fn test_mismatched_unclosed() {
        let input = "Unclosed $delimiter";
        let result = replace(input, ("$", "$"), ("$$", "$$")).unwrap_err();
        println!("{}", result);
        assert!(matches!(result, ConversionError::UnclosedDelimiter(9)));
    }

    #[test]
    fn test_empty_input() {
        let input = "";
        let result = replace(input, ("$", "$"), ("$$", "$$")).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_no_delimiters() {
        let input = "Hello, world!";
        let result = replace(input, ("$", "$"), ("$$", "$$")).unwrap();
        assert_eq!(result, "Hello, world!");
    }

    #[test]
    fn test_multiple_replacements() {
        let input = "$a$ then $$b$$ then $c$ and $$d$$";
        let result = replace(input, ("$", "$"), ("$$", "$$")).unwrap();
        assert_eq!(result, "[T1:a] then [T2:b] then [T1:c] and [T2:d]");
    }

    #[test]
    fn test_complete_replacements() {
        let input = "$a then b then c and d$";
        let result = replace(input, ("$", "$"), ("$$", "$$")).unwrap();
        assert_eq!(result, "[T1:a then b then c and d]");
    }

    #[test]
    fn test_mismatched_delimiters() {
        let input = "Mismatch $$ and $ signs";
        let result = replace(input, ("$", "$"), ("$$", "$$")).unwrap_err();
        println!("{}", result);
        assert!(matches!(
            result,
            ConversionError::MismatchedDelimiters(9, 16)
        ));
    }

    #[test]
    fn test_identical_delimiters() {
        let input = "|a| and ||b||";
        let result = replace(input, ("|", "|"), ("||", "||")).unwrap();
        assert_eq!(result, "[T1:a] and [T2:b]");
    }

    #[test]
    fn test_asymmetric_delimiters() {
        let input = r"let \(a=1\) and \[b=2\].";
        let result = replace(input, (r"\(", r"\)"), (r"\[", r"\]")).unwrap();
        assert_eq!(result, "let [T1:a=1] and [T2:b=2].");
    }

    #[test]
    fn test_asymmetric_delimiters_partial_delim() {
        let input = r"let\ \(a=1\) and \[b=2\].";
        let result = replace(input, (r"\(", r"\)"), (r"\[", r"\]")).unwrap();
        assert_eq!(result, "let\\ [T1:a=1] and [T2:b=2].");
    }

    #[test]
    fn test_asymmetric_delimiters_nested() {
        let input = r"let \(a=1 and \[b=2\]\).";
        let result = replace(input, (r"\(", r"\)"), (r"\[", r"\]")).unwrap_err();
        println!("{}", result);
        assert!(matches!(
            result,
            ConversionError::MismatchedDelimiters(4, 19)
        ));
    }

    #[test]
    fn test_asymmetric_delimiters_nested2() {
        let input = r"let \(a=1 and \[b=2\).";
        let result = replace(input, (r"\(", r"\)"), (r"\[", r"\]")).unwrap_err();
        println!("{}", result);
        assert!(matches!(result, ConversionError::NestedDelimiters(14)));
    }

    #[test]
    fn test_asymmetric_delimiters_nested3() {
        let input = r"let \(a=1 and \(b=2\).";
        let result = replace(input, (r"\(", r"\)"), (r"\[", r"\]")).unwrap_err();
        println!("{}", result);
        assert!(matches!(result, ConversionError::NestedDelimiters(14)));
    }

    #[test]
    fn test_asymmetric_delimiters_unclosed() {
        let input = r"let \(a=1 and b=2.";
        let result = replace(input, (r"\(", r"\)"), (r"\[", r"\]")).unwrap_err();
        println!("{}", result);
        assert!(matches!(result, ConversionError::UnclosedDelimiter(4)));
    }

    #[test]
    fn test_asymmetric_delimiters_dangling() {
        // We could make this an error, but it's sometimes useful to allow this.
        let input = r"let a=1\) and \(b=2\).";
        let result = replace(input, (r"\(", r"\)"), (r"\[", r"\]")).unwrap();
        assert_eq!(result, r"let a=1\) and [T1:b=2].");
    }

    #[test]
    fn test_asymmetric_delimiters_dangling2() {
        // We could make this an error, but it's sometimes useful to allow this.
        let input = r"let \(a=1\) and b=2\).";
        let result = replace(input, (r"\(", r"\)"), (r"\[", r"\]")).unwrap();
        assert_eq!(result, r"let [T1:a=1] and b=2\).");
    }

    #[test]
    fn test_multibyte_delimiters() {
        let input = "this is über ü(a=2ü).";
        let result = replace(input, ("ü(", "ü)"), ("ü[", "ü]")).unwrap();
        assert_eq!(result, "this is über [T1:a=2].");
    }

    #[test]
    fn test_long_delimiters() {
        let input = r#"based on its length, <span class="math inline">P(p)=2^{-len(p)}</span>, and then for a given
    <span class="math block">
    P(p)=2^{-len(p)}
    </span>
    Hello."#;
        let result = replace(
            input,
            ("<span class=\"math inline\">", "</span>"),
            ("<span class=\"math block\">", "</span>"),
        )
        .unwrap();
        assert_eq!(result, "based on its length, [T1:P(p)=2^{-len(p)}], and then for a given\n    [T2:\n    P(p)=2^{-len(p)}\n    ]\n    Hello.");
    }
}