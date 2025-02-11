#![allow(clippy::unnecessary_cast)]

/// Error types
#[rustfmt::skip] #[macro_use] pub mod error;
/// Bitmaps used for creating halftoned versions of contone buffers, and saving out
pub mod bitmap;
/// Dynamically allocated array of bytes
pub mod buffer;
/// Color params
pub mod color_params;
/// Colorspace
pub mod colorspace;
/// Context
pub mod context;
/// Provide two-way communication between application and library
pub mod cookie;
/// Destination
pub mod destination;
/// Device interface
pub mod device;
/// A way of packaging up a stream of graphical operations
pub mod display_list;
/// Common document operation interface
pub mod document;
/// Easy creation of new documents
pub mod document_writer;
/// Font
pub mod font;
/// Glyph
pub mod glyph;
/// Image
pub mod image;
/// Hyperlink
pub mod link;
/// Matrix operations
pub mod matrix;
/// Outline
pub mod outline;
/// Document page
pub mod page;
/// Path type
pub mod path;
/// PDF interface
pub mod pdf;
/// 2 dimensional array of contone pixels
pub mod pixmap;
/// Point type
pub mod point;
/// A representation for a region defined by 4 points
pub mod quad;
/// Rectangle types
pub mod rect;
/// Separations
pub mod separations;
/// Shadings
pub mod shade;
/// Size type
pub mod size;
/// Stroke state
pub mod stroke_state;

/// System font loading
#[cfg(feature = "system-fonts")]
pub mod system_font;
/// Text objects
pub mod text;
/// Text page
pub mod text_page;

pub use bitmap::Bitmap;
pub use buffer::Buffer;
pub use color_params::{ColorParams, RenderingIntent};
pub use colorspace::Colorspace;
pub(crate) use context::context;
pub use context::Context;
pub use cookie::Cookie;
pub use destination::{Destination, DestinationKind};
pub use device::{BlendMode, Device};
pub use display_list::DisplayList;
pub use document::{Document, MetadataName};
pub use document_writer::DocumentWriter;
pub(crate) use error::ffi_error;
pub use error::Error;
pub use font::{CjkFontOrdering, Font, SimpleFontEncoding, WriteMode};
pub use glyph::Glyph;
pub use image::Image;
pub use link::Link;
pub use matrix::Matrix;
pub use outline::Outline;
pub use page::Page;
pub use path::{Path, PathWalker};
pub use pixmap::{ImageFormat, Pixmap};
pub use point::Point;
pub use quad::Quad;
pub use rect::{IRect, Rect};
pub use separations::Separations;
pub use shade::Shade;
pub use size::Size;
pub use stroke_state::{LineCap, LineJoin, StrokeState};
pub use text::{Text, TextItem, TextSpan};
pub use text_page::{TextBlock, TextChar, TextLine, TextPage, TextPageOptions};

use core::marker::PhantomData;
use zerocopy::{FromBytes, IntoBytes};

unsafe fn rust_vec_from_ffi_ptr<F: IntoBytes, R: FromBytes>(
    ptr: *mut F,
    len: i32,
) -> Result<Vec<R>, Error> {
    let _assert = AssertSizeEquals::<F, R>::new();

    if ptr.is_null() {
        return Err(Error::UnexpectedNullPtr);
    }

    let rust_ty_ptr = ptr as *mut R;
    let len = usize::try_from(len)?;
    Ok(unsafe { Vec::from_raw_parts(rust_ty_ptr, len, len) })
}

fn rust_slice_to_ffi_ptr<F: FromBytes, R: IntoBytes>(vec: &[R]) -> Result<(*const F, i32), Error> {
    let _assert = AssertSizeEquals::<F, R>::new();

    let len = i32::try_from(vec.len())?;
    let ptr = vec.as_ptr() as *mut F;
    if ptr.is_null() {
        return Err(Error::UnexpectedNullPtr);
    }

    Ok((ptr, len))
}

struct AssertSizeEquals<A, B> {
    _phantom: PhantomData<(A, B)>,
}

impl<A, B> AssertSizeEquals<A, B> {
    const _SIZE_OK: () = assert!(size_of::<A>() == size_of::<B>());

    fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}
