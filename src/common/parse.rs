//! Extract and iterate over signed and unsigned integers embedded in text.
//!
//! A common pattern in Advent of Code is to parse numbers like `123`, `456` and `789`
//! from input resembling:
//!
//! ```none
//!   Lorem ipsum 123 dolor 456 sit 789 amet
//! ```
//!
//! This module provides methods to extract a single integer (`parse_uint`, `parse_int`)
//! or iterate over all integers (`parse_uint_iter`, `parse_int_iter`).
//!
//! Source: <https://github.com/maneatingape/advent-of-code-rust/blob/main/src/util/parse.rs>

use crate::common::integer::{Signed, Unsigned};
use std::marker::PhantomData;
use std::str::Bytes;

pub trait ParseByte {
    fn to_decimal(self) -> u8;
}

impl ParseByte for u8 {
    #[inline]
    fn to_decimal(self) -> u8 {
        self.wrapping_sub(b'0')
    }
}

pub struct ParseUnsigned<'a, T> {
    bytes: Bytes<'a>,
    phantom: PhantomData<T>,
}

pub struct ParseSigned<'a, T> {
    bytes: Bytes<'a>,
    phantom: PhantomData<T>,
}

pub trait ParseInteger {
    /// Parse the first unsigned integer in the string.
    ///
    /// ```
    /// use aoc::common::parse::*;
    /// let n: u32 = "abc123def".parse_uint();
    /// assert_eq!(n, 123);
    /// ```
    fn parse_uint<T: Unsigned<T>>(&self) -> T;

    /// Parse the first signed integer in the string.
    ///
    /// ```
    /// use aoc::common::parse::*;
    /// let n: i32 = "xx-42yy".parse_int();
    /// assert_eq!(n, -42);
    /// ```
    fn parse_int<T: Signed<T>>(&self) -> T;

    /// Return an iterator over all unsigned integers in the string.
    ///
    /// ```
    /// use aoc::common::parse::*;
    /// let nums: Vec<u32> = "x1 y22 z333".parse_uint_iter().collect();
    /// assert_eq!(nums, vec![1, 22, 333]);
    /// ```
    fn parse_uint_iter<T: Unsigned<T>>(&self) -> ParseUnsigned<'_, T>;

    /// Return an iterator over all signed integers in the string.
    ///
    /// ```
    /// use aoc::common::parse::*;
    /// let nums: Vec<i32> = "a-5 b6 c-7".parse_int_iter().collect();
    /// assert_eq!(nums, vec![-5, 6, -7]);
    /// ```
    fn parse_int_iter<T: Signed<T>>(&self) -> ParseSigned<'_, T>;
}

impl ParseInteger for &str {
    fn parse_uint<T: Unsigned<T>>(&self) -> T {
        match try_unsigned(&mut self.bytes()) {
            Some(t) => t,
            None => panic!("Unable to parse unsigned integer from: \"{self}\""),
        }
    }

    fn parse_int<T: Signed<T>>(&self) -> T {
        match try_signed(&mut self.bytes()) {
            Some(t) => t,
            None => panic!("Unable to parse signed integer from: \"{self}\""),
        }
    }

    fn parse_uint_iter<T: Unsigned<T>>(&self) -> ParseUnsigned<'_, T> {
        ParseUnsigned { bytes: self.bytes(), phantom: PhantomData }
    }

    fn parse_int_iter<T: Signed<T>>(&self) -> ParseSigned<'_, T> {
        ParseSigned { bytes: self.bytes(), phantom: PhantomData }
    }
}

impl<T: Unsigned<T>> Iterator for ParseUnsigned<'_, T> {
    type Item = T;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.bytes.size_hint();
        (lower / 3, upper.map(|u| u / 3))
    }

    fn next(&mut self) -> Option<Self::Item> {
        try_unsigned(&mut self.bytes)
    }
}

impl<T: Signed<T>> Iterator for ParseSigned<'_, T> {
    type Item = T;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.bytes.size_hint();
        (lower / 3, upper.map(|u| u / 3))
    }

    fn next(&mut self) -> Option<Self::Item> {
        try_signed(&mut self.bytes)
    }
}

fn try_unsigned<T: Unsigned<T>>(bytes: &mut Bytes<'_>) -> Option<T> {
    let mut n = loop {
        let byte = bytes.next()?;
        let digit = byte.to_decimal();

        if digit < 10 {
            break T::from(digit);
        }
    };

    loop {
        let Some(byte) = bytes.next() else {
            break Some(n);
        };
        let digit = byte.to_decimal();

        if digit < 10 {
            n = T::TEN * n + T::from(digit);
        } else {
            break Some(n);
        }
    }
}

fn try_signed<T: Signed<T>>(bytes: &mut Bytes<'_>) -> Option<T> {
    let (mut n, negative) = loop {
        let byte = bytes.next()?;
        let digit = byte.to_decimal();

        if digit == 253 {
            break (T::ZERO, true);
        }
        if digit < 10 {
            break (T::from(digit), false);
        }
    };

    loop {
        let Some(byte) = bytes.next() else {
            break Some(if negative { -n } else { n });
        };
        let digit = byte.to_decimal();

        if digit < 10 {
            n = T::TEN * n + T::from(digit);
        } else {
            break Some(if negative { -n } else { n });
        }
    }
}
