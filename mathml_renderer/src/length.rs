//! A length, stored in $\frac{1}{360}$ of an imperial point.
//!
//! This is also equal to $\frac{1}{480}$ of a CSS pixel, as demonstrated by
//! the following dimensional analysis and [CSS Values and Units 4]:
//!
//! $$`
//! 1 len \times \frac{360 point}{1 nip} \times \frac{1 inch}{72 point}
//! \times \frac{96 pixel}{1 inch} = 480 pixel
//! `$$
//!
//! [CSS values and units 4]:
//!     https://www.w3.org/TR/css-values-4/#absolute-lengths

use std::mem::MaybeUninit;
use std::num::{NonZeroI32, NonZeroU32};

#[cfg(feature = "serde")]
use serde::Serialize;

use crate::itoa::fmt_u32;

pub const PT_IN_LEN: NonZeroU32 = NonZeroU32::new(360).unwrap();
pub const PX_IN_LEN: NonZeroU32 = NonZeroU32::new(480).unwrap();

/// An absolute length.
/// See [the module][crate::length] for more information.
#[derive(Debug, Clone, Copy)]
pub struct Length(NonZeroI32);

impl Length {
    pub fn from_pt(pt: i32) -> Length {
        Length::from_value(pt * (PT_IN_LEN.get() as i32))
    }

    pub fn from_twip(twip: i32) -> Length {
        Length::from_value(twip * (PT_IN_LEN.get() as i32 / 10) / 2)
    }

    pub fn from_px(px: i32) -> Length {
        Length::from_value(px * (PX_IN_LEN.get() as i32))
    }

    pub fn from_value(value: i32) -> Length {
        // The problem that we are trying to solve here is that we want a niche in `Length`
        // but we also want to be able to represent 0, so we cannot just use `NonZeroI32` as is.
        // What we're doing here instead is adding `i32::MIN` to the plain value, such that
        // `i32::MIN` maps to 0, where the niche is. So, we can no longer represent `i32::MIN`,
        // but that's fine. We map the original value of `i32::MIN` to 1, which corresponds to
        // `i32::MIN + 1` in the original value space.
        Length(
            NonZeroI32::new(value.wrapping_add(i32::MIN))
                .unwrap_or(const { NonZeroI32::new(1).unwrap() }),
        )
    }

    fn value(self) -> i32 {
        i32::from(self.0).wrapping_sub(i32::MIN)
    }

    pub fn display_px(self, output: &mut String) {
        let value = self.value();
        write_impl(value, output, PX_IN_LEN, "px")
    }

    pub fn display_pt(self, output: &mut String) {
        let value = self.value();
        write_impl(value, output, PT_IN_LEN, "pt")
    }

    pub fn push_to_string(&self, output: &mut String) {
        let value = self.value();
        if value == 0 {
            output.push('0');
        } else {
            let (conv, unit) = if value % ((PT_IN_LEN.get() as i32) / 10) == 0
                || value % ((PX_IN_LEN.get() as i32) / 10) != 0
            {
                // If this measure can be serialized in 10ths of a point,
                // do that.
                (PT_IN_LEN, "pt")
            } else {
                (PX_IN_LEN, "px")
            };
            write_impl(value, output, conv, unit)
        }
    }
}

fn write_impl(value: i32, output: &mut String, conv: NonZeroU32, unit: &str) {
    if value < 0 {
        output.push('-');
    }
    let value = value.unsigned_abs();
    let mut buf = [MaybeUninit::uninit(); 10];
    output.push_str(fmt_u32(value / conv, &mut buf));
    let frac = value % conv;
    if frac != 0 {
        // only write two decimal points
        output.push('.');
        output.push_str(fmt_u32(frac * 10 / conv, &mut buf));
        let frac = (frac * 10) % conv;
        if frac != 0 {
            output.push_str(fmt_u32(frac * 10 / conv, &mut buf));
        }
    }
    output.push_str(unit)
}

#[cfg(feature = "serde")]
impl Serialize for Length {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i32(self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write() {
        let mut output = String::new();
        Length::from_pt(0).push_to_string(&mut output);
        assert_eq!(&output, "0");
        output.clear();
        Length::from_pt(1).push_to_string(&mut output);
        assert_eq!(&output, "1pt");
        output.clear();
        Length::from_pt(10).push_to_string(&mut output);
        assert_eq!(&output, "10pt");
        output.clear();
        Length::from_pt(5965232).push_to_string(&mut output);
        assert_eq!(&output, "5965232pt");
        output.clear();
        Length::from_pt(-5965232).push_to_string(&mut output);
        assert_eq!(&output, "-5965232pt");
        output.clear();
        Length::from_px(0).push_to_string(&mut output);
        assert_eq!(&output, "0");
        output.clear();
        Length::from_px(1).push_to_string(&mut output);
        assert_eq!(&output, "1px");
        output.clear();
        Length::from_px(10).push_to_string(&mut output);
        assert_eq!(&output, "10px");
        output.clear();
        Length::from_px(4473923).push_to_string(&mut output);
        assert_eq!(&output, "4473923px");
        output.clear();
        Length::from_px(-4473923).push_to_string(&mut output);
        assert_eq!(&output, "-4473923px");
    }
}
