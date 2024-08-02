use strum_macros::AsRefStr;

/// <mi> mathvariant attribute
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MathVariant {
    Normal = 1,
}

impl AsRef<str> for MathVariant {
    fn as_ref(&self) -> &str {
        match self {
            MathVariant::Normal => r#" mathvariant="normal""#,
        }
    }
}

#[derive(Debug, PartialEq, AsRefStr)]
pub enum Accent {
    #[strum(serialize = "true")]
    True,
    #[strum(serialize = "false")]
    False,
}

#[derive(Debug, PartialEq, AsRefStr)]
pub enum OpAttr {
    #[strum(serialize = r#" stretchy="true""#)]
    StretchyTrue = 1,
    #[strum(serialize = r#" stretchy="false""#)]
    StretchyFalse,
    #[strum(serialize = r#" movablelimits="false""#)]
    NoMovableLimits,
}

/// display style
#[derive(Debug, Clone, Copy, PartialEq, AsRefStr)]
pub enum FracAttr {
    #[strum(serialize = r#" displaystyle="true""#)]
    DisplayStyleTrue = 1,
    #[strum(serialize = r#" displaystyle="false""#)]
    DisplayStyleFalse,
    #[strum(serialize = r#" displaystyle="true" scriptlevel="0" style="padding-top: 0.1667em""#)]
    CFracStyle,
}

#[derive(Debug, Clone, Copy, PartialEq, AsRefStr)]
pub enum Style {
    #[strum(serialize = r#" displaystyle="true" scriptlevel="0""#)]
    DisplayStyle = 1,
    #[strum(serialize = r#" displaystyle="false" scriptlevel="0""#)]
    TextStyle,
    #[strum(serialize = r#" displaystyle="false" scriptlevel="1""#)]
    ScriptStyle,
    #[strum(serialize = r#" displaystyle="false" scriptlevel="2""#)]
    ScriptScriptStyle,
}

#[derive(Debug)]
pub enum Align {
    Center,
    Left,
    Alternating,
}

#[derive(Debug, AsRefStr)]
pub enum MathSpacing {
    #[strum(serialize = "0em")]
    Zero = 1,
    #[strum(serialize = "0.2222em")]
    FourMu, // 4/18 of an em/\quad
}

/// <mi> mathvariant attribute
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextTransform {
    Bold,
    BoldFraktur,
    BoldItalic,
    BoldSansSerif,
    BoldScript,
    DoubleStruck,
    Fraktur,
    // Initial,
    Italic,
    // Looped,
    Monospace,
    SansSerif,
    SansSerifBoldItalic,
    SansSerifItalic,
    Script,
    // Stretched,
    // Tailed,
}

fn add_offset(c: char, offset: u32) -> char {
    let new_char = char::from_u32(c as u32 + offset);
    debug_assert!(
        new_char.is_some(),
        "Invalid char: {}, offset: {}",
        c,
        offset
    );
    unsafe { new_char.unwrap_unchecked() }
}

impl TextTransform {
    #[allow(clippy::manual_is_ascii_check)]
    pub fn transform(&self, c: char) -> char {
        match self {
            TextTransform::BoldScript => match c {
                'A'..='Z' => add_offset(c, 0x1D48F),
                'a'..='z' => add_offset(c, 0x1D489),
                _ => c,
            },
            TextTransform::BoldItalic => match c {
                'A'..='Z' => add_offset(c, 0x1D427),
                'a'..='z' => add_offset(c, 0x1D421),
                'Α'..='Ω' => add_offset(c, 0x1D38B),
                'α'..='ω' => add_offset(c, 0x1D385),
                'ϴ' => '𝜭',
                '∇' => '𝜵',
                '∂' => '𝝏',
                'ϵ' => '𝝐',
                'ϑ' => '𝝑',
                'ϰ' => '𝝒',
                'ϕ' => '𝝓',
                'ϱ' => '𝝔',
                'ϖ' => '𝝕',
                _ => c,
            },
            TextTransform::Bold => match c {
                'A'..='Z' => add_offset(c, 0x1D3BF),
                'a'..='z' => add_offset(c, 0x1D3B9),
                'Α'..='Ω' => add_offset(c, 0x1D317),
                'α'..='ω' => add_offset(c, 0x1D311),
                'Ϝ'..='ϝ' => add_offset(c, 0x1D3EE),
                '0'..='9' => add_offset(c, 0x1D79E),
                'ϴ' => '𝚹',
                '∇' => '𝛁',
                '∂' => '𝛛',
                'ϵ' => '𝛜',
                'ϑ' => '𝛝',
                'ϰ' => '𝛞',
                'ϕ' => '𝛟',
                'ϱ' => '𝛠',
                'ϖ' => '𝛡',
                _ => c,
            },
            TextTransform::Fraktur => match c {
                'A'..='B' => add_offset(c, 0x1D4C3),
                'D'..='G' => add_offset(c, 0x1D4C3),
                'H'..='I' => add_offset(c, 0x20C4),
                'J'..='Q' => add_offset(c, 0x1D4C3),
                'S'..='Y' => add_offset(c, 0x1D4C3),
                'a'..='z' => add_offset(c, 0x1D4BD),
                'C' => 'ℭ',
                'R' => 'ℜ',
                'Z' => 'ℨ',
                _ => c,
            },
            TextTransform::Script => match c {
                'C'..='D' => add_offset(c, 0x1D45B),
                'E'..='F' => add_offset(c, 0x20EB),
                'H'..='I' => add_offset(c, 0x20C3),
                'J'..='K' => add_offset(c, 0x1D45B),
                'N'..='Q' => add_offset(c, 0x1D45B),
                'S'..='Z' => add_offset(c, 0x1D45B),
                'a'..='d' => add_offset(c, 0x1D455),
                'h'..='n' => add_offset(c, 0x1D455),
                'p'..='z' => add_offset(c, 0x1D455),
                'A' => '𝒜',
                'B' => 'ℬ',
                'G' => '𝒢',
                'L' => 'ℒ',
                'M' => 'ℳ',
                'R' => 'ℛ',
                'e' => 'ℯ',
                'f' => '𝒻',
                'g' => 'ℊ',
                'o' => 'ℴ',
                _ => c,
            },
            TextTransform::Monospace => match c {
                'A'..='Z' => add_offset(c, 0x1D62F),
                'a'..='z' => add_offset(c, 0x1D629),
                '0'..='9' => add_offset(c, 0x1D7C6),
                _ => c,
            },
            TextTransform::SansSerif => match c {
                'A'..='Z' => add_offset(c, 0x1D55F),
                'a'..='z' => add_offset(c, 0x1D559),
                '0'..='9' => add_offset(c, 0x1D7B2),
                _ => c,
            },
            TextTransform::BoldFraktur => match c {
                'A'..='Z' => add_offset(c, 0x1D52B),
                'a'..='z' => add_offset(c, 0x1D525),
                _ => c,
            },
            TextTransform::SansSerifBoldItalic => match c {
                'A'..='Z' => add_offset(c, 0x1D5FB),
                'a'..='z' => add_offset(c, 0x1D5F5),
                'Α'..='Ω' => add_offset(c, 0x1D3FF),
                'α'..='ω' => add_offset(c, 0x1D3F9),
                'ϴ' => '𝞡',
                '∇' => '𝞩',
                '∂' => '𝟃',
                'ϵ' => '𝟄',
                'ϑ' => '𝟅',
                'ϰ' => '𝟆',
                'ϕ' => '𝟇',
                'ϱ' => '𝟈',
                'ϖ' => '𝟉',
                _ => c,
            },
            TextTransform::SansSerifItalic => match c {
                'A'..='Z' => add_offset(c, 0x1D5C7),
                'a'..='z' => add_offset(c, 0x1D5C1),
                _ => c,
            },
            TextTransform::BoldSansSerif => match c {
                'A'..='Z' => add_offset(c, 0x1D593),
                'a'..='z' => add_offset(c, 0x1D58D),
                'Α'..='Ω' => add_offset(c, 0x1D3C5),
                'α'..='ω' => add_offset(c, 0x1D3BF),
                '0'..='9' => add_offset(c, 0x1D7BC),
                'ϴ' => '𝝧',
                '∇' => '𝝯',
                '∂' => '𝞉',
                'ϵ' => '𝞊',
                'ϑ' => '𝞋',
                'ϰ' => '𝞌',
                'ϕ' => '𝞍',
                'ϱ' => '𝞎',
                'ϖ' => '𝞏',
                _ => c,
            },
            TextTransform::DoubleStruck => match c {
                'A'..='B' => add_offset(c, 0x1D4F7),
                'D'..='G' => add_offset(c, 0x1D4F7),
                'I'..='M' => add_offset(c, 0x1D4F7),
                'P'..='Q' => add_offset(c, 0x20C9),
                'S'..='Y' => add_offset(c, 0x1D4F7),
                'a'..='z' => add_offset(c, 0x1D4F1),
                '0'..='9' => add_offset(c, 0x1D7A8),
                'C' => 'ℂ',
                'H' => 'ℍ',
                'N' => 'ℕ',
                'R' => 'ℝ',
                'Z' => 'ℤ',
                _ => c,
            },
            TextTransform::Italic => match c {
                'A'..='Z' => add_offset(c, 0x1D3F3),
                'a'..='g' => add_offset(c, 0x1D3ED),
                'i'..='z' => add_offset(c, 0x1D3ED),
                'Α'..='Ω' => add_offset(c, 0x1D351),
                'α'..='ω' => add_offset(c, 0x1D34B),
                'h' => 'ℎ',
                'ı' => '𝚤',
                'ȷ' => '𝚥',
                'ϴ' => '𝛳',
                '∇' => '𝛻',
                '∂' => '𝜕',
                'ϵ' => '𝜖',
                'ϑ' => '𝜗',
                'ϰ' => '𝜘',
                'ϕ' => '𝜙',
                'ϱ' => '𝜚',
                'ϖ' => '𝜛',
                _ => c,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TextTransform;

    #[test]
    fn transform_test() {
        let problems = [
            ('G', TextTransform::BoldScript, '𝓖'),
            ('H', TextTransform::Italic, '𝐻'),
            ('X', TextTransform::Fraktur, '𝔛'),
            ('S', TextTransform::Script, '𝒮'),
            ('f', TextTransform::Bold, '𝐟'),
            ('g', TextTransform::Bold, '𝐠'),
            ('o', TextTransform::DoubleStruck, '𝕠'),
            ('D', TextTransform::Monospace, '𝙳'),
            ('x', TextTransform::Monospace, '𝚡'),
            ('2', TextTransform::Monospace, '𝟸'),
            ('U', TextTransform::SansSerif, '𝖴'),
            ('v', TextTransform::SansSerif, '𝗏'),
            ('4', TextTransform::SansSerif, '𝟦'),
            ('A', TextTransform::SansSerifBoldItalic, '𝘼'),
            ('a', TextTransform::SansSerifBoldItalic, '𝙖'),
            ('Α', TextTransform::SansSerifBoldItalic, '𝞐'),
            ('α', TextTransform::SansSerifBoldItalic, '𝞪'),
            ('A', TextTransform::SansSerifItalic, '𝘈'),
            ('a', TextTransform::SansSerifItalic, '𝘢'),
            ('J', TextTransform::BoldSansSerif, '𝗝'),
            ('r', TextTransform::BoldSansSerif, '𝗿'),
            ('Ξ', TextTransform::BoldSansSerif, '𝝣'),
            ('τ', TextTransform::BoldSansSerif, '𝞃'),
        ];
        for (source, transform, target) in problems.into_iter() {
            assert_eq!(
                target,
                transform.transform(source),
                "executed: {:?}({})",
                transform,
                source
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn as_array(c: char) -> [u8; 4] {
        let mut buf = [0; 4];
        let len = c.len_utf8();
        c.encode_utf8(&mut buf[(4 - len)..]);
        buf
    }

    fn add_mut(a: [u8; 4], b: &[u8; 4]) -> [u8; 4] {
        let mut a = a.map(|x| x);
        for i in 0..4 {
            a[i] = a[i].wrapping_add(b[i]);
        }
        a
    }

    fn as_str(a: &[u8; 4]) -> &str {
        // Skip leading zeros.
        // The following is one instruction: "movbel  (%rdi), %eax"
        let value = u32::from_be_bytes(*a);
        // SSE4 and WASM have a lzcnt instruction.
        let offset = (value.leading_zeros() / 8) as usize;
        std::str::from_utf8(&a[offset..]).unwrap()
    }

    #[test]
    fn check_utf8() {
        let tf = TextTransform::Italic;
        for c in ['A', 'B', 'C'].into_iter() {
            println!("{:?}", as_array(c));
            assert_eq!(
                as_array(tf.transform(c)),
                add_mut(as_array(c), &[240, 157, 144, 115])
            );
        }
        for c in ['Α', 'Β', 'Γ'].into_iter() {
            println!("{:?}", as_array(c));
            assert_eq!(
                as_array(tf.transform(c)),
                add_mut(as_array(c), &[240, 157, 205, 17])
            );
        }
    }

    #[test]
    fn test_double_struck() {
        let tf = TextTransform::DoubleStruck;
        for c in ['A', 'B'].into_iter() {
            println!("{:?}", as_array(c));
            assert_eq!(
                as_array(tf.transform(c)),
                add_mut(as_array(c), &[240, 157, 148, 119])
            );
        }
        for c in ['P', 'Q'].into_iter() {
            println!("{:?}", as_array(c));
            assert_eq!(
                as_array(tf.transform(c)),
                add_mut(as_array(c), &[0, 226, 132, 73])
            );
        }
    }

    #[test]
    fn test_range() {
        let c = as_array('F');
        let b = u32::from_be_bytes(c);
        assert_eq!(b, 70);
        assert!(65 <= b);
        assert!(b <= 90);
        let c = as_array('Γ');
        let b = u32::from_be_bytes(c);
        assert_eq!(b, 52883);
        assert!(52881 <= b);
        assert!(b <= 52905);
    }

    #[test]
    fn test_as_str() {
        let c = as_array('F');
        assert_eq!(as_str(&c), "F");
        let c = as_array('Γ');
        assert_eq!(as_str(&c), "Γ");
        let c = as_array('\u{0}');
        assert_eq!(as_str(&c), "");
        let c = as_array('𝜛');
        assert_eq!(as_str(&c), "𝜛");
    }
}
