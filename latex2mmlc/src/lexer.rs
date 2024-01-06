//! Lexer
//!
//! - Input: `String`
//! - Output: `Vec<Token>`
//!

use crate::ops::Op;
use crate::{ops, token::Token};

/// Lexer
#[derive(Debug, Clone)]
pub(crate) struct Lexer<'a> {
    input: std::str::CharIndices<'a>,
    cur: char,
    offset: usize,
    input_string: &'a str,
    input_length: usize,
}

impl<'a> Lexer<'a> {
    /// Receive the input source code and generate a LEXER instance.
    pub(crate) fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input: input.char_indices(),
            cur: '\u{0}',
            offset: 0,
            input_string: input,
            input_length: input.len(),
        };
        lexer.read_char();
        lexer
    }

    /// One character progresses.
    fn read_char(&mut self) {
        (self.offset, self.cur) = self.input.next().unwrap_or((self.input_length, '\u{0}'));
    }

    /// Skip blank characters.
    fn skip_whitespace(&mut self) {
        while matches!(self.cur, ' ' | '\t' | '\n' | '\r') {
            self.read_char();
        }
    }

    /// Read one command to a token.
    #[inline]
    fn read_command(&mut self) -> &'a str {
        // Always read at least one character.
        self.read_char();
        let start = self.offset;

        if !self.cur.is_ascii_alphabetic() {
            self.read_char();
            // SAFETY: we got `start` and `offset` from `CharIndices`, so they are valid bounds.
            return unsafe { self.input_string.get_unchecked(start..self.offset) };
        }

        // Read in all ASCII characters.
        self.read_char();
        while self.cur.is_ascii_alphabetic() {
            self.read_char();
        }
        // SAFETY: we got `start` and `offset` from `CharIndices`, so they are valid bounds.
        unsafe { self.input_string.get_unchecked(start..self.offset) }
    }

    /// Read one number into a token.
    fn read_number(&mut self) -> (&'a str, Op) {
        let start = self.offset;
        while self.cur.is_ascii_digit() || matches!(self.cur, '.' | ',') {
            // Before we accept the current character, we need to check the next one.
            let candidate = self.cur;
            let end = self.offset;
            self.read_char();
            if !candidate.is_ascii_digit() && !self.cur.is_ascii_digit() {
                // If neither the candiate character nor the next character is a digit,
                // we stop.
                // But we need to return the `candidate` character.
                let number = unsafe { self.input_string.get_unchecked(start..end) };
                return (number, Op(candidate));
            }
        }
        let number = unsafe { self.input_string.get_unchecked(start..self.offset) };
        (number, ops::NULL)
    }

    /// Read text until the next `}`.
    pub(crate) fn read_text_content(&mut self, whitespace: WhiteSpace) -> Option<String> {
        let mut text = String::new();
        if matches!(whitespace, WhiteSpace::Skip) {
            self.skip_whitespace();
        }
        while self.cur != '}' {
            if self.cur == '\u{0}' {
                return None;
            }
            if matches!(whitespace, WhiteSpace::Convert) && self.cur == ' ' {
                text.push('\u{A0}')
            } else {
                text.push(self.cur);
            }
            self.read_char();
            if matches!(whitespace, WhiteSpace::Skip) {
                self.skip_whitespace();
            }
        }
        self.read_char(); // Discard the closing brace.
        Some(text)
    }

    /// Generate the next token.
    pub(crate) fn next_token(&mut self, wants_digit: bool) -> Token<'a> {
        if wants_digit && self.cur.is_ascii_digit() {
            let start = self.offset;
            self.read_char();
            let num = unsafe { self.input_string.get_unchecked(start..self.offset) };
            return Token::Number(num, ops::NULL);
        }
        self.skip_whitespace();

        let token = match self.cur {
            '=' => Token::Operator(ops::EQUAL),
            ';' => Token::Operator(ops::SEMICOLON),
            ',' => Token::Operator(ops::COMMA),
            '.' => Token::Operator(ops::DOT),
            '\'' => Token::Operator(ops::APOS),
            '(' => Token::Paren(ops::LEFT_PARENTHESIS),
            ')' => Token::Paren(ops::RIGHT_PARENTHESIS),
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '[' => Token::Paren(ops::LEFT_SQUARE_BRACKET),
            ']' => Token::Paren(ops::RIGHT_SQUARE_BRACKET),
            '|' => Token::Paren(ops::VERTICAL_LINE),
            '+' => Token::Operator(ops::PLUS),
            '-' => Token::Operator(ops::MINUS),
            '*' => Token::Operator(ops::ASTERISK),
            '/' => Token::Operator(ops::SOLIDUS),
            '!' => Token::Operator(ops::EXCLAMATION_MARK),
            '<' => Token::OpLessThan,
            '>' => Token::OpGreaterThan,
            '_' => Token::Underscore,
            '^' => Token::Circumflex,
            '&' => Token::Ampersand,
            '~' => Token::NonBreakingSpace,
            '\u{0}' => Token::EOF,
            ':' => Token::Colon,
            ' ' => Token::Letter('\u{A0}'),
            '\\' => {
                return Token::from_command(self.read_command());
            }
            c => {
                if c.is_ascii_digit() {
                    let (num, op) = self.read_number();
                    return Token::Number(num, op);
                } else if c.is_ascii_alphabetic() {
                    Token::Letter(c)
                } else {
                    Token::NormalLetter(c)
                }
            }
        };
        self.read_char();
        token
    }
}

pub(crate) enum WhiteSpace {
    Skip,
    Record,
    Convert,
}

#[cfg(test)]
mod tests {
    use super::super::token::Token;
    use super::*;

    #[test]
    fn lexer_test() {
        let problems = vec![
            (r"3", vec![Token::Number("3", ops::NULL)]),
            (r"3.14", vec![Token::Number("3.14", ops::NULL)]),
            (r"3.14.", vec![Token::Number("3.14", ops::DOT)]),
            (
                r"3..14",
                vec![
                    Token::Number("3", ops::DOT),
                    Token::Operator(ops::DOT),
                    Token::Number("14", ops::NULL),
                ],
            ),
            (r"x", vec![Token::Letter('x')]),
            (r"\pi", vec![Token::Letter('π')]),
            (
                r"x = 3.14",
                vec![
                    Token::Letter('x'),
                    Token::Operator(ops::EQUAL),
                    Token::Number("3.14", ops::NULL),
                ],
            ),
            (r"\alpha\beta", vec![Token::Letter('α'), Token::Letter('β')]),
            (
                r"x+y",
                vec![
                    Token::Letter('x'),
                    Token::Operator(ops::PLUS),
                    Token::Letter('y'),
                ],
            ),
            (
                r"\ 1",
                vec![Token::Space("1"), Token::Number("1", ops::NULL)],
            ),
        ];

        for (problem, answer) in problems.iter() {
            let mut lexer = Lexer::new(problem);
            for answer in answer.iter() {
                assert_eq!(&lexer.next_token(false), answer);
            }
        }
    }
}
