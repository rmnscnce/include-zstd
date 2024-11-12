//! # `include_zstd`
//!
//! This crate provides a macro that allows you to include Zstd-compressed data at compile time. The compressed data is stored in an [`EmbeddedZstd`] struct, which can be converted into a `Vec<u8>` or a `Box<[u8]>` for decompression.
//!
//! ## Example
//! ```rust
//! use include_zstd::{EmbeddedZstd, include_zstd};
//!
//! // Create an instance of `EmbeddedZstd` using the `include_zstd!` macro
//! const COMPRESSED_DATA: EmbeddedZstd<4538> = include_zstd!("data/udhr_en.txt", 19);
//!
//! // The compressed data can be converted into a `Vec<u8>` or a `Box<[u8]>`
//! let compressed_data_vec: Vec<u8> = COMPRESSED_DATA.into();
//! let compressed_data_box: Box<[u8]> = COMPRESSED_DATA.into();
//!
//! // The compressed data can also be compared to other instances of `EmbeddedZstd`
//! const OTHER_COMPRESSED_DATA: EmbeddedZstd<4538> = include_zstd!("data/udhr_en.txt", 19);
//! assert!(COMPRESSED_DATA == OTHER_COMPRESSED_DATA);
//! ```
//!

use std::io::Read;

use ruzstd::StreamingDecoder;

#[doc(hidden)]
pub extern crate include_zstd_macro;

/// # `EmbeddedZstd`
/// Opaque struct that holds Zstd-compressed data.
///
/// See [`include_zstd!`] for information on how to create an instance of this struct.
///
/// ## Usage
/// ```rust
/// use include_zstd::{EmbeddedZstd, include_zstd};
///
/// // Create an instance of `EmbeddedZstd` using the `include_zstd!` macro
/// const COMPRESSED_DATA: EmbeddedZstd<4538> = include_zstd!("data/udhr_en.txt", 19);
///
/// // The compressed data can be converted into a `Vec<u8>` or a `Box<[u8]>`
/// let compressed_data_vec: Vec<u8> = COMPRESSED_DATA.into();
/// let compressed_data_box: Box<[u8]> = COMPRESSED_DATA.into();
///
/// // The compressed data can also be compared to other instances of `EmbeddedZstd`
/// const OTHER_COMPRESSED_DATA: EmbeddedZstd<4538> = include_zstd!("data/udhr_en.txt", 19);
/// assert!(COMPRESSED_DATA == OTHER_COMPRESSED_DATA);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EmbeddedZstd<const SIZE: usize>([u8; SIZE]);

impl<const SIZE: usize> EmbeddedZstd<SIZE> {
    #[doc(hidden)]
    #[must_use]
    pub const unsafe fn new_unchecked(data: [u8; SIZE]) -> Self {
        Self(data)
    }

    /// Returns the size of the compressed data, in bytes.
    pub const fn size(&self) -> usize {
        SIZE
    }
}

impl<const SIZE: usize> From<EmbeddedZstd<SIZE>> for Vec<u8> {
    /// Decompress the data and return it as a `Vec<u8>`
    fn from(value: EmbeddedZstd<SIZE>) -> Self {
        let mut dec = StreamingDecoder::new(value.0.as_ref()).unwrap();
        let mut buf = Vec::new();

        dec.read_to_end(&mut buf).unwrap();
        buf
    }
}

impl<const SIZE: usize> From<EmbeddedZstd<SIZE>> for Box<[u8]> {
    /// Decompress the data and return it as a `Box<[u8]>`
    fn from(value: EmbeddedZstd<SIZE>) -> Self {
        Vec::from(value).into_boxed_slice()
    }
}

/// # `include_zstd!`
///
/// Compress and include a file at compile time using Zstd with the specified compression level and return an [`EmbeddedZstd`] struct.
///
/// ## Usage
/// ```rust
/// use include_zstd::{EmbeddedZstd, include_zstd};
///
/// // Use `const` bindings to include the compressed data
/// const COMPRESSED_DATA: EmbeddedZstd<4538> = include_zstd!("data/udhr_en.txt", 19);
///
/// // It's also possible to use `let` bindings to eliminate the need for the type annotation
/// let compressed_data = include_zstd!("data/udhr_en.txt", 19);
///
/// // Regardless of the method used, the representation of the compressed data is the same
/// assert!(COMPRESSED_DATA == compressed_data);
/// ```
#[macro_export]
macro_rules! include_zstd {
    ($path:literal, $compression_level:literal) => {
        $crate::include_zstd_macro::include_zstd_inner!($path, $compression_level)
    };
}
