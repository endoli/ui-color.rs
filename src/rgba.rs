// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! RGBA Colors

use std::convert::From;
#[cfg(feature = "webrender")]
use webrender_traits;

#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    components: [f32; 4],
}

impl Color {
    #[allow(missing_docs)]
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color { components: [r, g, b, a] }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn red(&self) -> f32 {
        self.components[0]
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn green(&self) -> f32 {
        self.components[1]
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn blue(&self) -> f32 {
        self.components[2]
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn alpha(&self) -> f32 {
        self.components[3]
    }
}

impl super::Color for Color {
    fn components(&self) -> &[f32] {
        &self.components
    }

    fn component_count(&self) -> usize {
        4
    }
}

#[cfg(feature = "webrender")]
impl From<Color> for webrender_traits::ColorF {
    fn from(c: Color) -> Self {
        webrender_traits::ColorF {
            r: c.components[0],
            g: c.components[1],
            b: c.components[2],
            a: c.components[3],
        }
    }
}

#[cfg(feature = "webrender")]
impl From<Color> for webrender_traits::ColorU {
    fn from(c: Color) -> Self {
        webrender_traits::ColorF {
                r: c.components[0],
                g: c.components[1],
                b: c.components[2],
                a: c.components[3],
            }
            .into()
    }
}

#[cfg(feature = "webrender")]
impl From<webrender_traits::ColorF> for Color {
    fn from(c: webrender_traits::ColorF) -> Self {
        Color { components: [c.r, c.g, c.b, c.a] }
    }
}

#[cfg(feature = "webrender")]
impl From<webrender_traits::ColorU> for Color {
    fn from(c: webrender_traits::ColorU) -> Self {
        let cf: webrender_traits::ColorF = c.into();
        Color { components: [cf.r, cf.g, cf.b, cf.a] }
    }
}

// TODO: impl From<hsla::Color> for Color.

#[cfg(test)]
mod tests {
    use super::Color;
    #[cfg(feature = "webrender")]
    use webrender_traits;

    #[test]
    fn color_new() {
        let c = Color::new(1.0f32, 1.0f32, 1.0f32, 1.0f32);
        assert_eq!(c.components, [1.0f32, 1.0f32, 1.0f32, 1.0f32]);
    }

    #[cfg(feature = "webrender")]
    #[test]
    fn webrender_coloru_conversion() {
        let c = Color::new(1.0f32, 0.0, 0.0, 1.0f32);
        let wrtcu = webrender_traits::ColorU::new(255, 0, 0, 255);
        assert_eq!(wrtcu, c.into());
        assert_eq!(c, wrtcu.into());
    }
}
