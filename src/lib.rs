// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # UI Color

#![warn(missing_docs)]
#![deny(trivial_numeric_casts,
        unsafe_code, unstable_features,
        unused_import_braces, unused_qualifications)]

#[cfg(feature = "webrender")]
extern crate webrender_traits;

pub mod cmyk;
pub mod hsla;
pub mod hsva;
pub mod rgba;

#[allow(missing_docs)]
pub trait Color {
    fn components(&self) -> &[f32];
    fn component_count(&self) -> usize;
    // TODO: What else?
}

#[allow(missing_docs)]
pub fn hsl(h: f32, s: f32, l: f32) -> hsla::Color {
    hsla::Color::new(h, s, l, 1.0)
}

#[allow(missing_docs)]
pub fn hsla(h: f32, s: f32, l: f32, a: f32) -> hsla::Color {
    hsla::Color::new(h, s, l, a)
}

#[allow(missing_docs)]
pub fn rgb(r: f32, g: f32, b: f32) -> rgba::Color {
    rgba::Color::new(r, g, b, 1.0)
}

#[allow(missing_docs)]
pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> rgba::Color {
    rgba::Color::new(r, g, b, a)
}
