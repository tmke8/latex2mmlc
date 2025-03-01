use std::mem::MaybeUninit;

pub const MAX_DEC_N: usize = u32::MAX.ilog(10) as usize + 1;

/// Formats a `u32` into a decimal string.
///
/// This code has been essentially copied from `library/core/src/fmt/num.rs` in the `std` library.
/// Why do this instead of using `format!`? Because using `format!` pulls in a lot of dependencies
/// and make our code on WASM much larger.
///
/// There is discussion here: https://github.com/rust-lang/libs-team/issues/546 to expose the
/// below functionality directly in the `std` library, but nothing has been decided yet.
pub fn fmt_u32(mut n: u32, buf: &mut [MaybeUninit<u8>; MAX_DEC_N]) -> &str {
    let mut curr = MAX_DEC_N;
    let buf_ptr = buf.as_mut_ptr() as *mut u8;

    // SAFETY: To show that it's OK to copy into `buf_ptr`, notice that at the beginning
    // `curr == buf.len() == 10 > log(n)` since `n < 2^32 < 10^10`, and at
    // each step this is kept the same as `n` is divided. Since `n` is always
    // non-negative, this means that `curr > 0` so `buf_ptr[curr..curr + 1]`
    // is safe to access.
    unsafe {
        loop {
            debug_assert!(curr > 0);
            curr -= 1;
            buf_ptr.add(curr).write((n % 10) as u8 + b'0');
            n /= 10;

            if n == 0 {
                break;
            }
        }
    }

    // SAFETY: `curr` >= 0 (since we made `buf` large enough), and all the chars are valid UTF-8
    unsafe {
        debug_assert!(buf.len() > curr);
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(
            buf_ptr.add(curr),
            buf.len() - curr,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fmt_u32() {
        let mut buf = [MaybeUninit::uninit(); 10];
        assert_eq!(fmt_u32(0, &mut buf), "0");
        assert_eq!(fmt_u32(1, &mut buf), "1");
        assert_eq!(fmt_u32(10, &mut buf), "10");
        assert_eq!(fmt_u32(1234567890, &mut buf), "1234567890");
        assert_eq!(fmt_u32(u32::MAX, &mut buf), "4294967295");
    }
}
