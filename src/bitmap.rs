use std::slice;
use std::{convert::TryFrom, ptr::NonNull};

use mupdf_sys::*;

use crate::{context, Error, Pixmap};

/// Bitmaps have 1 bit per component.
/// Only used for creating halftoned versions of contone buffers, and saving out.
/// Samples are stored msb first, akin to pbms.
#[derive(Debug)]
pub struct Bitmap {
    pub(crate) inner: NonNull<fz_bitmap>,
}

impl Bitmap {
    pub fn from_pixmap(pixmap: &Pixmap) -> Result<Self, Error> {
        let inner = ffi_try!(mupdf_new_bitmap_from_pixmap(context(), pixmap.inner));
        let inner = NonNull::new(inner).ok_or(Error::UnexpectedNullPtr)?;
        Ok(Self { inner })
    }

    fn inner_as_ref(&self) -> &fz_bitmap {
        unsafe { self.inner.as_ref() }
    }

    /// Width of the region in pixels.
    pub fn width(&self) -> u32 {
        self.inner_as_ref().w as u32
    }

    /// Height of the region in pixels.
    pub fn height(&self) -> u32 {
        self.inner_as_ref().h as u32
    }

    pub fn stride(&self) -> i32 {
        self.inner_as_ref().stride
    }

    pub fn n(&self) -> i32 {
        self.inner_as_ref().n
    }

    /// Horizontal and vertical resolution in dpi (dots per inch).
    pub fn resolution(&self) -> (i32, i32) {
        let x_res = self.inner_as_ref().xres;
        let y_res = self.inner_as_ref().yres;
        (x_res, y_res)
    }

    pub fn samples(&self) -> &[u8] {
        let len = (self.width() * self.height()) as usize;
        unsafe { slice::from_raw_parts(self.inner_as_ref().samples, len) }
    }

    pub fn samples_mut(&mut self) -> &mut [u8] {
        let len = (self.width() * self.height()) as usize;
        unsafe { slice::from_raw_parts_mut(self.inner_as_ref().samples, len) }
    }
}

impl Drop for Bitmap {
    fn drop(&mut self) {
        unsafe { fz_drop_bitmap(context(), self.inner.as_ptr()) }
    }
}

impl TryFrom<Pixmap> for Bitmap {
    type Error = Error;

    fn try_from(pixmap: Pixmap) -> Result<Self, Self::Error> {
        Self::from_pixmap(&pixmap)
    }
}

#[cfg(test)]
mod test {
    use crate::{Bitmap, Colorspace, Pixmap};

    #[test]
    fn test_new_bitmap() {
        let cs = Colorspace::device_gray();
        let mut pixmap = Pixmap::new_with_w_h(&cs, 100, 100, false).expect("Pixmap::new_with_w_h");
        pixmap.clear().unwrap();
        let bitmap = Bitmap::from_pixmap(&pixmap).unwrap();
        assert_eq!(bitmap.width(), 100);
        assert_eq!(bitmap.n(), 1);
    }

    #[test]
    fn test_new_bitmap_error() {
        let cs = Colorspace::device_rgb();
        let mut pixmap = Pixmap::new_with_w_h(&cs, 100, 100, false).expect("Pixmap::new_with_w_h");
        pixmap.clear().unwrap();
        assert!(Bitmap::from_pixmap(&pixmap).is_err());
    }
}
