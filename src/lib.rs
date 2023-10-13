//! # Memdbg
//!
//! Memdbg provides the [`Buf`] struct which implements Debug
//! to provide a peak into memory akin to hex readers.
//!
//! The [`buf_dbg`] macro extends this view to any struct.
//!
//! ## Example
//!
//! ```rust
//! let buf = memdbg::Buf(*b"\x41 \x68 \x90 \x00 \x2f This is a test string, The Line Break will demonstrate multiline formatting!");
//! dbg!("{buf:?}");
//! ```
//! will display
//! ```text
//! | 41 20 68 20 90 20 00 20 | 2F 20 54 68 69 73 20 69 | 73 20 61 20 74 65 73 74 | 20 73 74 72 69 6E 67 2C | A . h . . . . . / . T h i s . i s . a . t e s t . s t r i n g ,
//! | 20 54 68 65 20 4C 69 6E | 65 20 42 72 65 61 6B 20 | 77 69 6C 6C 20 64 65 6D | 6F 6E 73 74 72 61 74 65 | . T h e . L i n e . B r e a k . w i l l . d e m o n s t r a t e
//! | 20 6D 75 6C 74 69 6C 69 | 6E 65 20 66 6F 72 6D 61 | 74 74 69 6E 67 21                                 | . m u l t i l i n e . f o r m a t t i n g !
//! ```

#![no_std]

use core::{
    cmp::min,
    fmt::{Debug, Formatter},
};

/// A buffer of `N` bytes, a `[u8; N]` with a nice Debug impl
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
#[repr(C)]
pub struct Buf<const N: usize>(pub [u8; N]);

impl<const N: usize> Debug for Buf<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let align = core::mem::align_of::<usize>();
        let chunks_per_line = 32 / align;
        let ptr = self.0.as_ptr();
        let offset = ptr.align_offset(align);
        let offset = min(offset, self.0.len());
        let (pre, rest) = self.0.split_at(offset);
        for &char in pre {
            f.write_fmt(format_args!(" {:02X}", char))?;
        }
        if !pre.is_empty() {
            f.write_str(" |")?;
        }
        write_ascii(f, pre)?;
        for line in rest.chunks(align * chunks_per_line) {
            f.write_str("\n")?;
            for bytes in line.chunks(align) {
                f.write_str(" |")?;
                for &char in bytes {
                    f.write_fmt(format_args!(" {:02X}", char))?;
                }
            }

            let fill = align * chunks_per_line - line.len();
            let fill = 3 * fill + 2 * (fill / align);
            for _ in 0..fill {
                f.write_str(" ")?;
            }

            f.write_str(" | ")?;

            write_ascii(f, line)?;
        }

        Ok(())
    }
}

fn write_ascii(f: &mut Formatter, buf: &[u8]) -> core::fmt::Result {
    for &char in buf {
        if char.is_ascii_graphic() {
            f.write_fmt(format_args!("{}", char as char))?;
        } else {
            f.write_str(".")?;
        }
    }

    Ok(())
}

#[doc(hidden)]
#[cfg(feature = "debug")]
pub fn dbg_impl<T, const N: usize>(
    f: &mut ::core::fmt::Formatter<'_>,
    t: &T,
    _name: &str,
) -> ::core::fmt::Result {
    <Buf<N> as Debug>::fmt(unsafe { core::mem::transmute(t) }, f)
}

#[doc(hidden)]
#[cfg(all(not(feature = "debug"), feature = "stringify"))]
pub fn dbg_impl<T, const N: usize>(
    f: &mut ::core::fmt::Formatter<'_>,
    _t: &T,
    name: &str,
) -> ::core::fmt::Result {
    f.write_str(name)
}

/// Implements Debug for a type.
///
/// Feature `debug` formats it as if it would be a [`Buf`].
/// Otherwise, if feature `stringify` is specified, it displays the type name.
/// If none of those features are specified, this macro does nothing and does
/// not implement Debug.
#[macro_export]
#[cfg(any(feature = "debug", feature = "stringify"))]
macro_rules! buf_dbg {
    ($name:ident) => {
        impl ::core::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::memdbg::dbg_impl::<Self, { core::mem::size_of::<Self>() }>(
                    f,
                    self,
                    stringify!($name),
                )
            }
        }
    };
}

/// Doesn't do anything without any features specified.
#[macro_export]
#[cfg(not(any(feature = "debug", feature = "stringify")))]
macro_rules! buf_dbg {
    ($name:ident) => {};
}
