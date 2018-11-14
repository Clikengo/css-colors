use std::fmt;

pub mod angle;
pub mod ratio;
use angle::Angle;
use ratio::Ratio;

/// A trait that can be used for converting between different color models
/// and performing various transformations on them.
pub trait Color {
    type Alpha: Color;

    /// Converts `self` to its CSS string format.
    ///
    /// # Examples
    /// ```
    /// use css_colors::{Color, RGB, RGBA};
    ///
    /// let salmon = RGB::new(250, 128, 114);
    /// let opaque_salmon = RGBA::new(250, 128, 114, 128);
    ///
    /// assert_eq!(salmon.to_css(), "rgb(250, 128, 114)");
    /// assert_eq!(opaque_salmon.to_css(), "rgba(250, 128, 114, 0.50)");
    /// ```
    fn to_css(self) -> String;

    /// Converts `self` into its RGB representation.
    /// When converting from a color model that supports an alpha channel
    /// (e.g. RGBA), the alpha value will not be preserved.
    ///
    /// # Examples
    /// ```
    /// use css_colors::{Color, RGB, RGBA};
    ///
    /// let opaque_tomato = RGBA::new(255, 99, 71, 128);
    ///
    /// assert_eq!(opaque_tomato.to_rgb(), RGB::new(255, 99, 71));
    /// ```
    fn to_rgb(self) -> RGB;

    /// Converts `self` into its RGBA representation.
    /// When converting from a color model that does not supports an alpha channel
    /// (e.g. RGB), it will be treated as fully opaque.
    ///
    /// # Examples
    /// ```
    /// use css_colors::{Color, RGB, RGBA};
    ///
    /// let tomato = RGB::new(255, 99, 71);
    ///
    /// assert_eq!(tomato.to_rgba(), RGBA::new(255, 99, 71, 255));
    /// ```
    fn to_rgba(self) -> RGBA;

    /// Converts `self` into its HSL representation.
    /// When converting from a color model that supports an alpha channel
    /// (e.g. RGBA), the alpha value will not be preserved.
    ///
    /// # Examples
    /// ```
    /// use css_colors::{Color, RGB, RGBA, HSL};
    ///
    /// let tomato = RGB::new(255, 99, 71);
    /// let opaque_tomato = RGBA::new(255, 99, 71, 128);
    ///
    /// assert_eq!(tomato.to_hsl(), HSL::new(9, 100, 64));
    /// assert_eq!(opaque_tomato.to_hsl(), HSL::new(9, 100, 64));
    /// ```
    fn to_hsl(self) -> HSL;

    /// Converts `self` into its HSLA representation.
    /// When converting from a color model that does not supports an alpha channel
    /// (e.g. RGB), it will be treated as fully opaque.
    ///
    /// # Examples
    /// ```
    /// use css_colors::{Color, RGB, RGBA, HSLA};
    ///
    /// let tomato = RGB::new(255, 99, 71);
    /// let opaque_tomato = RGBA::new(255, 99, 71, 128);
    ///
    /// assert_eq!(tomato.to_hsla(), HSLA::new(9, 100, 64, 255));
    /// assert_eq!(opaque_tomato.to_hsla(), HSLA::new(9, 100, 64, 128));
    /// ```
    fn to_hsla(self) -> HSLA;

    /// Increases the saturation of `self` by an absolute amount.
    /// Operates on the color within its HSL representation and preserves any existing alpha channel.
    /// For more, see Less' [Color Operations](http://lesscss.org/functions/#color-operations-saturate).
    ///
    /// # Examples
    /// ```
    /// use css_colors::{Color, RGB, HSLA};
    ///
    /// let salmon = HSLA::new(6, 93, 71, 255);
    /// let cornflower_blue = RGB::new(100, 149, 237);
    ///
    /// assert_eq!(salmon.saturate(7), HSLA::new(6, 100, 71, 255));
    /// assert_eq!(cornflower_blue.saturate(10), RGB::new(92, 146, 246));
    /// ```
    fn saturate(self, amount: u8) -> Self;

    /// Decreases the saturation of `self` by an absolute amount.
    /// Operates on the color within its HSL representation and preserves any existing alpha channel.
    /// For more, see Less' [Color Operations](http://lesscss.org/functions/#color-operations-desaturate).
    ///
    /// # Examples
    /// ```
    /// use css_colors::{Color, RGB, RGBA};
    ///
    /// let tomato = RGBA::new(255, 99, 71, 255);
    /// let cornflower_blue = RGB::new(100, 149, 237);
    ///
    /// assert_eq!(tomato.desaturate(10), RGBA::new(246, 105, 80, 255));
    /// assert_eq!(cornflower_blue.desaturate(33), RGB::new(129, 157, 209));
    /// ```
    fn desaturate(self, amount: u8) -> Self;

    /// Increases the lightness of `self` by an absolute amount.
    /// Operates on the color within its HSL representation and preserves any existing alpha channel.
    /// For more, see Less' [Color Operations](http://lesscss.org/functions/#color-operations-lighten).
    ///
    /// # Examples
    /// ```
    /// use css_colors::{Color, RGB, RGBA};
    ///
    /// let tomato = RGBA::new(255, 99, 71, 255);
    /// let cornflower_blue = RGB::new(100, 149, 237);
    ///
    /// assert_eq!(tomato.lighten(20), RGBA::new(255, 185, 173, 255));
    /// assert_eq!(cornflower_blue.lighten(33), RGB::new(251, 253, 255));
    /// ```
    fn lighten(self, amount: u8) -> Self;

    /// Decreases the lightness of `self` by an absolute amount.
    /// Operates on the color within its HSL representation and preserves any existing alpha channel.
    /// For more, see Less' [Color Operations](http://lesscss.org/functions/#color-operations-darken).
    ///
    /// # Examples
    /// ```
    /// use css_colors::{Color, RGB, RGBA};
    ///
    /// let tomato = RGBA::new(255, 99, 71, 255);
    /// let cornflower_blue = RGB::new(100, 149, 237);
    ///
    /// assert_eq!(tomato.darken(20), RGBA::new(224, 34, 0, 255));
    /// assert_eq!(cornflower_blue.darken(33), RGB::new(18, 65, 152));
    /// ```
    fn darken(self, amount: u8) -> Self;

    /// Decreases the transparency (or increase the opacity) of `self`, making it more opaque.
    /// Has no effect on opaque (non-alpha) colors.
    /// For more, see Less' [Color Operations](http://lesscss.org/functions/#color-operations-fadein).
    ///
    /// # Examples
    /// ```
    /// use css_colors::{Color, RGB, RGBA};
    ///
    /// let tomato = RGBA::new(255, 99, 71, 64);
    /// let cornflower_blue = RGB::new(100, 149, 237);
    ///
    /// assert_eq!(tomato.fadein(64), RGBA::new(255, 99, 71, 128));
    /// assert_eq!(cornflower_blue.fadein(128), RGB::new(100, 149, 237));
    /// ```
    fn fadein(self, amount: u8) -> Self;

    /// Increases the transparency (or decrease the opacity) of `self`, making it less opaque.
    /// Has no effect on opaque (non-alpha) colors.
    /// For more, see Less' [Color Operations](http://lesscss.org/functions/#color-operations-fadeout).
    ///
    /// # Examples
    /// ```
    /// use css_colors::{Color, RGB, RGBA};
    ///
    /// let tomato = RGBA::new(255, 99, 71, 128);
    /// let cornflower_blue = RGB::new(100, 149, 237);
    ///
    /// assert_eq!(tomato.fadeout(64), RGBA::new(255, 99, 71, 64));
    /// assert_eq!(cornflower_blue.fadeout(128), RGB::new(100, 149, 237));
    /// ```
    fn fadeout(self, amount: u8) -> Self;

    /// Sets the absolute opacity of `self`.
    /// Can be applied to colors whether they already have an opacity value or not.
    /// For more, see Less' [Color Operations](http://lesscss.org/functions/#color-operations-fade).
    ///
    /// # Examples
    /// ```
    /// use css_colors::{Color, RGB, RGBA};
    ///
    /// let tomato = RGBA::new(255, 99, 71, 128);
    /// let cornflower_blue = RGB::new(100, 149, 237);
    ///
    /// assert_eq!(tomato.fade(25), RGBA::new(255, 99, 71, 25));
    /// assert_eq!(cornflower_blue.fade(128), RGBA::new(100, 149, 237, 128));
    /// ```
    fn fade(self, amount: u8) -> Self::Alpha;

    /// Rotate the hue angle of `self` in either direction.
    /// Returns the appropriate `RGB` representation of the color once it has been spun.
    /// For more, see Less' [Color Operations](http://lesscss.org/functions/#color-operations-spin).
    ///
    /// # Examples
    /// ```
    /// use css_colors::{Color, RGB, HSL};
    ///
    /// let red = HSL::new(10, 90, 50);
    /// let golden = RGB::new(243, 166, 13);
    /// let pink = RGB::new(243, 13, 90);
    ///
    /// assert_eq!(red.spin(30), golden);
    /// assert_eq!(red.spin(-30), pink);
    /// ```
    fn spin(self, amount: i16) -> RGB;

    /// Mixes two colors (`self` and any other `RGBA` color) together in variable proportion.
    /// Takes opacity into account in the calculations.
    /// Optionally takes a percentage balance point between the two colors, and defaults to 50%.
    /// Returns the appropriate `RGBA` representation of the color once it has been mixed.
    /// For more, see Less' [Color Operations](http://lesscss.org/functions/#color-operations-mix).
    ///
    /// # Examples
    /// ```
    /// use css_colors::{Color, RGB, RGBA, HSL};
    ///
    /// let red = HSL::new(10, 90, 50);
    /// let golden = RGB::new(243, 166, 13);
    /// let navy = RGBA::new(0, 0, 80, 255);
    ///
    /// assert_eq!(red.mix(navy, 50), RGBA::new(122, 26, 47, 255));
    /// assert_eq!(golden.mix(navy, 25), RGBA::new(61, 42, 63, 255));
    /// ```
    fn mix<T: Color>(self, other: T, weight: u8) -> Self::Alpha;

    /// Mixes `self` with white in variable proportion.
    /// Equivalent to calling `mix()` with `white` (`rgb(255, 255, 255)`).
    /// Optionally takes a percentage balance point between `self` and `white`, defaults to 50%.
    /// Returns the appropriate `RGBA` representation of the color once it has been mixed.
    /// For more, see Less' [Color Operations](http://lesscss.org/functions/#color-operations-tint).
    ///
    /// # Examples
    /// ```
    /// use css_colors::{Color, RGB, RGBA, HSL};
    ///
    /// let red = HSL::new(10, 90, 50);
    /// let golden = RGB::new(243, 166, 13);
    ///
    /// assert_eq!(red.tint(50), RGBA::new(249, 153, 134, 255));
    /// assert_eq!(golden.tint(25), RGBA::new(252, 233, 194, 255));
    /// ```
    fn tint(self, weight: u8) -> RGBA;

    /// Mixes `self` with white in variable proportion.
    /// Equivalent to calling `mix()` with `black` (`rgb(0, 0, 0)`).
    /// Optionally takes a percentage balance point between `self` and `black`, defaults to 50%.
    /// Returns the appropriate `RGBA` representation of the color once it has been mixed.
    /// For more, see Less' [Color Operations](http://lesscss.org/functions/#color-operations-shade).
    ///
    /// # Examples
    /// ```
    /// use css_colors::{Color, RGB, RGBA, HSL};
    ///
    /// let red = HSL::new(10, 90, 50);
    /// let golden = RGB::new(243, 166, 13);
    ///
    /// assert_eq!(red.shade(50), RGBA::new(122, 26, 7, 255));
    /// assert_eq!(golden.shade(25), RGBA::new(61, 42, 3, 255));
    /// ```
    fn shade(self, weight: u8) -> RGBA;

    /// Remove all saturation from `self` in the HSL color space.
    /// Equivalent to calling `desaturate(0)` on a color.
    /// For more, see Less' [Color Operations](http://lesscss.org/functions/#color-operations-greyscale).
    ///
    /// # Examples
    /// ```
    /// use css_colors::{Color, RGB, RGBA};
    ///
    /// let tomato = RGBA::new(255, 99, 71, 255);
    /// let cornflower_blue = RGB::new(100, 149, 237);
    ///
    /// assert_eq!(tomato.greyscale(), RGBA::new(163, 163, 163, 255));
    /// assert_eq!(cornflower_blue.greyscale(), RGB::new(169, 169, 169));
    /// ```
    fn greyscale(self) -> Self;
}

#[derive(Debug, Copy, Clone, PartialEq)]
/// A struct to represent how much red, green, and blue should be added to create a color.
///
/// Valid values for r, g, and b must be a u8 between `0-255`, represented as a `Ratio`.
///
/// For more, see the [CSS Color Spec](https://www.w3.org/TR/2018/REC-css-color-3-20180619/#rgb-color).
pub struct RGB {
    // red
    pub r: Ratio,

    // green
    pub g: Ratio,

    // blue
    pub b: Ratio,
}

impl fmt::Display for RGB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "rgb({}, {}, {})",
            self.r.as_u8(),
            self.g.as_u8(),
            self.b.as_u8()
        )
    }
}

impl RGB {
    /// Transforms numerical values into an RGB struct.
    ///
    /// # Example
    /// ```
    /// use {css_colors::RGB, css_colors::ratio::Ratio as Ratio};
    ///
    /// let salmon = RGB::new(250, 128, 114);
    ///
    /// assert_eq!(salmon, RGB { r: Ratio::from_u8(250), g: Ratio::from_u8(128), b: Ratio::from_u8(114) });
    /// ```
    pub fn new(r: u8, g: u8, b: u8) -> RGB {
        RGB {
            r: Ratio::from_u8(r),
            g: Ratio::from_u8(g),
            b: Ratio::from_u8(b),
        }
    }
}

impl Color for RGB {
    type Alpha = RGBA;

    fn to_css(self) -> String {
        self.to_string()
    }

    fn to_rgb(self) -> RGB {
        self
    }

    fn to_rgba(self) -> RGBA {
        RGBA::new(self.r.as_u8(), self.g.as_u8(), self.b.as_u8(), 255)
    }

    /// The algorithm for converting from rgb to hsl format, which determines
    /// the equivalent luminosity, saturation, and hue.
    fn to_hsl(self) -> HSL {
        let RGB { r, g, b } = self;

        // If r, g, and b are the same, the color is a shade of grey (between
        // black and white), with no hue or saturation. In that situation, there
        // is no saturation or hue, and we can use any value to determine luminosity.
        if r == g && g == b {
            return HSL {
                h: Angle::new(0),             // h
                s: Ratio::from_percentage(0), // s
                l: r,                         // l
            };
        }

        // Otherwise, to determine luminosity, we conver the RGB values into a
        // percentage value, find the max and the min of those values, sum them
        // together, and divide by 2.
        let r = self.r.as_f32();
        let g = self.g.as_f32();
        let b = self.b.as_f32();

        // let max = vec![r, g, b].iter().max().to_f32()
        let max = if r > g && r > b {
            r
        } else if g > b {
            g
        } else {
            b
        };

        let min = if r < g && r < b {
            r
        } else if g < b {
            g
        } else {
            b
        };

        let luminosity = (max + min) / 2.0;

        // To find the saturation, we look at the max and min values.
        // If the max and the min are the same, there is no saturation to the color.
        // Otherwise, we calculate the saturation based on if the luminosity is
        // greater than or less than 0.5.
        let saturation = if max == min {
            0.0
        } else if luminosity < 0.5 {
            (max - min) / (max + min)
        } else {
            (max - min) / (2.0 - (max + min))
        };

        // To calculate the hue, we look at which value (r, g, or b) is the max.
        // Based on that, we subtract the difference between the other two values,
        // adding 2.0 or 4.0 to account for the degrees on the color wheel, and
        // then dividing that by the difference between the max and the min values.
        // Finally, we multiply the hue value by 60 to convert it to degrees on
        // the color wheel, accounting for negative hues as well.
        let mut hue = if max == r {
            (g - b) / (max - min)
        } else if max == g {
            2.0 + (b - r) / (max - min)
        } else {
            4.0 + (r - g) / (max - min)
        };

        hue *= 60.0;

        // If hue is negative, add 360 to make it it positive.
        if hue <= 0.0 {
            hue += 360.0;
        }

        HSL {
            h: Angle::new(hue.round() as u16),
            s: Ratio::from_f32(saturation),
            l: Ratio::from_f32(luminosity),
        }
    }

    fn to_hsla(self) -> HSLA {
        let HSL { h, s, l } = self.to_hsl();

        HSLA::new(h.degrees(), s.as_percentage(), l.as_percentage(), 255)
    }

    fn saturate(self, amount: u8) -> Self {
        self.to_hsl().saturate(amount).to_rgb()
    }

    fn desaturate(self, amount: u8) -> Self {
        self.to_hsl().desaturate(amount).to_rgb()
    }

    fn lighten(self, amount: u8) -> Self {
        self.to_hsl().lighten(amount).to_rgb()
    }

    fn darken(self, amount: u8) -> Self {
        self.to_hsl().darken(amount).to_rgb()
    }

    fn fadein(self, _amount: u8) -> Self {
        self
    }

    fn fadeout(self, _amount: u8) -> Self {
        self
    }

    fn fade(self, amount: u8) -> RGBA {
        RGBA::new(self.r.as_u8(), self.g.as_u8(), self.b.as_u8(), amount)
    }

    fn spin(self, amount: i16) -> Self {
        self.to_hsl().spin(amount).to_rgb()
    }

    fn mix<T: Color>(self, other: T, weight: u8) -> RGBA {
        self.to_rgba().mix(other, weight)
    }

    fn tint(self, _weight: u8) -> RGBA {
        self.to_rgba().tint(_weight)
    }

    fn shade(self, _weight: u8) -> RGBA {
        self.to_rgba().shade(_weight)
    }

    fn greyscale(self) -> Self {
        self.to_hsl().greyscale().to_rgb()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
/// A struct to represent how much red, green, and blue should be added to create a color.
/// Also handles alpha specifications.
///
/// Valid values for r, g, and b must be a u8 between `0-255`, represented as a `Ratio`.
/// Alpha (a) values must fall between `0-255`.
///
/// For more, see the [CSS Color Spec](https://www.w3.org/TR/2018/REC-css-color-3-20180619/#rgba-color).
pub struct RGBA {
    // red
    pub r: Ratio,

    // green
    pub g: Ratio,

    // blue
    pub b: Ratio,

    // alpha
    pub a: Ratio,
}

impl fmt::Display for RGBA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "rgba({}, {}, {}, {:.02})",
            self.r.as_u8(),
            self.g.as_u8(),
            self.b.as_u8(),
            self.a.as_f32()
        )
    }
}

impl RGBA {
    /// Transforms numerical values into an RGBA struct.
    ///
    /// # Example
    /// ```
    /// use {css_colors::RGBA, css_colors::ratio::Ratio as Ratio};
    ///
    /// let light_salmon = RGBA::new(250, 128, 114, 128);
    ///
    /// assert_eq!(light_salmon, RGBA { r: Ratio::from_u8(250), g: Ratio::from_u8(128), b: Ratio::from_u8(114), a: Ratio::from_u8(128) });
    /// ```
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> RGBA {
        RGBA {
            r: Ratio::from_u8(r),
            g: Ratio::from_u8(g),
            b: Ratio::from_u8(b),
            a: Ratio::from_u8(a),
        }
    }
}

impl Color for RGBA {
    type Alpha = Self;

    fn to_css(self) -> String {
        self.to_string()
    }

    fn to_rgb(self) -> RGB {
        RGB::new(self.r.as_u8(), self.g.as_u8(), self.b.as_u8())
    }

    fn to_rgba(self) -> RGBA {
        self
    }

    fn to_hsl(self) -> HSL {
        self.to_rgb().to_hsl()
    }

    fn to_hsla(self) -> HSLA {
        let HSL { h, s, l } = self.to_hsl();
        HSLA::new(
            h.degrees(),
            s.as_percentage(),
            l.as_percentage(),
            self.a.as_u8(),
        )
    }

    fn saturate(self, amount: u8) -> Self {
        self.to_hsla().saturate(amount).to_rgba()
    }

    fn desaturate(self, amount: u8) -> Self {
        self.to_hsla().desaturate(amount).to_rgba()
    }

    fn lighten(self, amount: u8) -> Self {
        self.to_hsla().lighten(amount).to_rgba()
    }

    fn darken(self, amount: u8) -> Self {
        self.to_hsla().darken(amount).to_rgba()
    }

    fn fadein(self, amount: u8) -> Self {
        let RGBA { r, g, b, a } = self;

        RGBA {
            r,
            g,
            b,
            a: a + Ratio::from_u8(amount),
        }
    }

    fn fadeout(self, amount: u8) -> Self {
        let RGBA { r, g, b, a } = self;

        RGBA {
            r,
            g,
            b,
            a: a - Ratio::from_u8(amount),
        }
    }

    fn fade(self, amount: u8) -> Self {
        let RGBA { r, g, b, a: _ } = self;

        RGBA {
            r,
            g,
            b,
            a: Ratio::from_u8(amount),
        }
    }

    fn spin(self, amount: i16) -> RGB {
        self.to_hsl().spin(amount).to_rgb()
    }

    // This algorithm takes into account both the user-provided weight (w) and
    // the difference between the alpha values of the two colors (a) to determine
    // the weighted average of the two colors.
    // Taken from Sass's implementation (http://sass-lang.com/documentation/Sass/Script/Functions.html#mix-instance_method)
    fn mix<T: Color>(self, other: T, weight: u8) -> Self {
        let RGBA {
            r: r_lhs,
            g: g_lhs,
            b: b_lhs,
            a: a_lhs,
        } = self;

        let RGBA {
            r: r_rhs,
            g: g_rhs,
            b: b_rhs,
            a: a_rhs,
        } = other.to_rgba();

        let ratio_weight = Ratio::from_percentage(weight);

        // Convert weight into a decimal, and then scale it so that it falls between a range of [-1, 1].
        let w = (ratio_weight.as_f32() * 2.0) - 1.0;

        // Find the difference between the left and right side's alphas (somewhere between [-1, 1]).
        let a = a_lhs.as_f32() - a_rhs.as_f32();

        // Find the combined rgb_weight, taking into account the user's passed-in weight and alpha (range of [-1, 1]).
        let rgb_weight = if w * a == -1.0 {
            w
        } else {
            (w + a) / (1.0 + w * a)
        };

        // Find the combined rgb weight, scaling it to fall in a range bewtween [0, 1].
        let rgb_weight = (rgb_weight + 1.0) / 2.0;

        // Convert left and right side's weights into Ratios.
        let rgb_weight_lhs = Ratio::from_f32(rgb_weight);
        let rgb_weight_rhs = Ratio::from_f32(1.0) - rgb_weight_lhs;

        let alpha_weight_lhs = ratio_weight;
        let alpha_weight_rhs = Ratio::from_f32(1.0) - alpha_weight_lhs;

        RGBA {
            r: (r_lhs * rgb_weight_lhs) + (r_rhs * rgb_weight_rhs),
            g: (g_lhs * rgb_weight_lhs) + (g_rhs * rgb_weight_rhs),
            b: (b_lhs * rgb_weight_lhs) + (b_rhs * rgb_weight_rhs),
            a: (a_lhs * alpha_weight_lhs) + (a_rhs * alpha_weight_rhs),
        }
    }

    fn tint(self, _weight: u8) -> RGBA {
        self.mix(RGBA::new(255, 255, 255, 255), _weight)
    }

    fn shade(self, _weight: u8) -> RGBA {
        self.mix(RGBA::new(0, 0, 0, 255), _weight)
    }

    fn greyscale(self) -> Self {
        self.to_hsl().greyscale().to_rgba()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
/// A struct to represent how much hue, saturation, and luminosity should be added to create a color.
/// The hue is a degree on the color wheel; 0 (or 360) is red, 120 is green, 240 is blue.
/// A valid value for `h` must range between `0-360`.
/// The saturation ranges between `0-100`, where `0` is completely desaturated, and `100` is full saturation.
/// The luminosity ranges between `0-100`, where `0` is no light (black), and `100` is full light (white).
///
/// For more, see the [CSS Color Spec](https://www.w3.org/TR/2018/REC-css-color-3-20180619/#hsl-color).
pub struct HSL {
    // hue
    pub h: Angle,

    // saturation
    pub s: Ratio,

    // luminosity
    pub l: Ratio,
}

impl fmt::Display for HSL {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "hsl({}, {}, {})", self.h.degrees(), self.s, self.l)
    }
}

impl HSL {
    /// Transforms numerical values into a HSL struct.
    ///
    /// # Example
    /// ```
    /// use {css_colors::HSL, css_colors::angle::Angle as Angle, css_colors::ratio::Ratio as Ratio};
    ///
    /// let salmon = HSL::new(6, 93, 71);
    ///
    /// assert_eq!(salmon, HSL { h: Angle::new(6), s: Ratio::from_percentage(93), l: Ratio::from_percentage(71) });
    /// ```
    pub fn new(h: u16, s: u8, l: u8) -> HSL {
        HSL {
            h: Angle::new(h),
            s: Ratio::from_percentage(s),
            l: Ratio::from_percentage(l),
        }
    }
}

impl Color for HSL {
    type Alpha = HSLA;

    fn to_css(self) -> String {
        self.to_string()
    }

    fn to_rgb(self) -> RGB {
        let hue = self.h;
        let s = self.s.as_f32();
        let l = self.l.as_f32();

        // If there is no saturation, the color is a shade of grey.
        // We can convert the luminosity and set r, g, and b to that value.
        if s == 0.0 {
            return RGB {
                r: self.l,
                g: self.l,
                b: self.l,
            };
        }

        // If the color is not a grey, then we need to create a temporary variable to continue with the algorithm.
        // If the luminosity is less than 50%, we add 1.0 to the saturation and multiply by the luminosity.
        // Otherwise, we add the luminosity and saturation, and subtract the product of luminosity and saturation from it.
        let temp_1 = if l < 0.5 {
            l * (1.0 + s)
        } else {
            (l + s) - (l * s)
        };

        // Another temporary variable.
        let temp_2 = (2.0 * l) - temp_1;

        // Create a rotation of 120 degrees in order to divide the angle into thirds.
        let rotation = Angle::new(120);

        // Then rotate the circle clockwise by 1/3 for the red value, and by 2/3rds for the blue value.
        let temporary_r = (hue + rotation).degrees();
        let temporary_g = hue.degrees();
        let temporary_b = (hue - rotation).degrees();

        let red = to_rgb_value(temporary_r, temp_1, temp_2);
        let green = to_rgb_value(temporary_g, temp_1, temp_2);
        let blue = to_rgb_value(temporary_b, temp_1, temp_2);

        RGB {
            r: Ratio::from_f32(red),
            g: Ratio::from_f32(green),
            b: Ratio::from_f32(blue),
        }
    }

    fn to_rgba(self) -> RGBA {
        let RGB { r, g, b } = self.to_rgb();

        RGBA::new(r.as_u8(), g.as_u8(), b.as_u8(), 255)
    }

    fn to_hsl(self) -> HSL {
        self
    }

    fn to_hsla(self) -> HSLA {
        HSLA::new(
            self.h.degrees(),
            self.s.as_percentage(),
            self.l.as_percentage(),
            255,
        )
    }

    fn saturate(self, amount: u8) -> Self {
        let HSL { h, s, l } = self;

        HSL {
            h,
            s: s + Ratio::from_percentage(amount),
            l,
        }
    }

    fn desaturate(self, amount: u8) -> Self {
        let HSL { h, s, l } = self;

        HSL {
            h,
            s: s - Ratio::from_percentage(amount),
            l,
        }
    }

    fn lighten(self, amount: u8) -> Self {
        let HSL { h, s, l } = self;

        HSL {
            h,
            s,
            l: l + Ratio::from_percentage(amount),
        }
    }

    fn darken(self, amount: u8) -> Self {
        let HSL { h, s, l } = self;

        HSL {
            h,
            s,
            l: l - Ratio::from_percentage(amount),
        }
    }

    fn fadein(self, _amount: u8) -> Self {
        self
    }

    fn fadeout(self, _amount: u8) -> Self {
        self
    }

    fn fade(self, amount: u8) -> Self::Alpha {
        let HSL { h, s, l } = self;

        HSLA {
            h,
            s,
            l,
            a: Ratio::from_u8(amount),
        }
    }

    fn spin(self, amount: i16) -> RGB {
        let HSL { h, s, l } = self;

        assert!(amount < 360, "Invalid spin amount");

        let new_hue = if amount.is_negative() {
            h - Angle::new((amount * -1) as u16)
        } else {
            h + Angle::new(amount as u16)
        };

        HSL { h: new_hue, s, l }.to_rgb()
    }

    fn mix<T: Color>(self, other: T, weight: u8) -> Self::Alpha {
        self.to_hsla().mix(other, weight)
    }

    fn tint(self, _weight: u8) -> RGBA {
        self.to_rgba().tint(_weight)
    }

    fn shade(self, _weight: u8) -> RGBA {
        self.to_rgba().shade(_weight)
    }

    fn greyscale(self) -> Self {
        let HSL { h, s: _, l } = self;

        HSL {
            h,
            s: Ratio::from_percentage(0),
            l,
        }
    }
}

// A function to convert an HSL value (either h, s, or l) into the equivalent, valid RGB value.
fn to_rgb_value(val: u16, temp_1: f32, temp_2: f32) -> f32 {
    let value = val as f32 / 360.0;

    if value > (2.0 / 3.0) {
        // value > 0.66667
        temp_2
    } else if value > (1.0 / 2.0) {
        // value is between 0.5 and 0.66667
        temp_2 + ((temp_1 - temp_2) * ((2.0 / 3.0) - value) * 6.0)
    } else if value > (1.0 / 6.0) {
        // value is between 0.16667 and 0.5
        temp_1
    } else {
        // value <= 0.16667
        temp_2 + ((temp_1 - temp_2) * value * 6.0)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
/// A struct to represent how much hue, saturation, and luminosity should be added to create a color.
/// Also handles alpha specifications.
///
/// A valid value for `h` must range between `0-360`.
/// The saturation ranges between `0-100`, where `0` is completely desaturated, and `100` is full saturation.
/// The luminosity ranges between `0-100`, where `0` is no light (black), and `100` is full light (white).
///
/// For more, see the [CSS Color Spec](https://www.w3.org/TR/2018/REC-css-color-3-20180619/#hsla-color).
pub struct HSLA {
    // hue
    pub h: Angle,

    // saturation
    pub s: Ratio,

    // luminosity
    pub l: Ratio,

    // alpha
    pub a: Ratio,
}

impl fmt::Display for HSLA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "hsla({}, {}, {}, {:.02})",
            self.h,
            self.s,
            self.l,
            self.a.as_f32()
        )
    }
}

impl HSLA {
    /// Transforms numerical values into a HSL struct.
    ///
    /// # Example
    /// ```
    /// use {css_colors::HSLA, css_colors::angle::Angle as Angle, css_colors::ratio::Ratio as Ratio};
    /// let light_salmon = HSLA::new(6, 93, 71, 128);
    ///
    /// assert_eq!(light_salmon, HSLA { h: Angle::new(6), s: Ratio::from_percentage(93), l: Ratio::from_percentage(71), a: Ratio::from_percentage(50) });
    /// ```
    pub fn new(h: u16, s: u8, l: u8, a: u8) -> HSLA {
        HSLA {
            h: Angle::new(h),
            s: Ratio::from_percentage(s),
            l: Ratio::from_percentage(l),
            a: Ratio::from_u8(a),
        }
    }
}

impl Color for HSLA {
    type Alpha = Self;

    fn to_css(self) -> String {
        self.to_string()
    }

    fn to_rgb(self) -> RGB {
        self.to_hsl().to_rgb()
    }

    fn to_rgba(self) -> RGBA {
        let RGB { r, g, b } = self.to_rgb();
        RGBA { r, g, b, a: self.a }
    }

    fn to_hsl(self) -> HSL {
        let HSLA { h, s, l, .. } = self;
        HSL { h, s, l }
    }

    fn to_hsla(self) -> HSLA {
        self
    }

    fn saturate(self, amount: u8) -> Self {
        let HSLA { h, s, l, a } = self;

        HSLA {
            h,
            s: s + Ratio::from_percentage(amount),
            l,
            a,
        }
    }

    fn desaturate(self, amount: u8) -> Self {
        let HSLA { h, s, l, a } = self;

        HSLA {
            h,
            s: s - Ratio::from_percentage(amount),
            l,
            a,
        }
    }

    fn lighten(self, amount: u8) -> Self {
        let HSLA { h, s, l, a } = self;

        HSLA {
            h,
            s,
            l: l + Ratio::from_percentage(amount),
            a,
        }
    }

    fn darken(self, amount: u8) -> Self {
        let HSLA { h, s, l, a } = self;

        HSLA {
            h,
            s,
            l: l - Ratio::from_percentage(amount),
            a,
        }
    }

    fn fadein(self, amount: u8) -> Self {
        let HSLA { h, s, l, a } = self;

        HSLA {
            h,
            s,
            l,
            a: a + Ratio::from_u8(amount),
        }
    }

    fn fadeout(self, amount: u8) -> Self {
        let HSLA { h, s, l, a } = self;

        HSLA {
            h,
            s,
            l,
            a: a - Ratio::from_u8(amount),
        }
    }

    fn fade(self, amount: u8) -> Self::Alpha {
        let HSLA { h, s, l, .. } = self;

        HSLA {
            h,
            s,
            l,
            a: Ratio::from_u8(amount),
        }
    }

    fn spin(self, amount: i16) -> RGB {
        self.to_hsl().spin(amount).to_rgb()
    }

    fn mix<T: Color>(self, other: T, weight: u8) -> Self::Alpha {
        self.to_rgba().mix(other, weight).to_hsla()
    }

    fn tint(self, _weight: u8) -> RGBA {
        self.to_rgba().tint(_weight)
    }

    fn shade(self, _weight: u8) -> RGBA {
        self.to_rgba().shade(_weight)
    }

    fn greyscale(self) -> Self {
        self.to_hsl().greyscale().to_hsla()
    }
}

#[cfg(test)]
mod css_color_tests {
    use {Angle, Color, Ratio, HSL, HSLA, RGB, RGBA};

    #[test]
    fn can_create_color_structs() {
        assert_eq!(
            RGB::new(5, 10, 15),
            RGB {
                r: Ratio::from_u8(5),
                g: Ratio::from_u8(10),
                b: Ratio::from_u8(15),
            }
        );
        assert_eq!(
            RGBA::new(5, 10, 15, 255),
            RGBA {
                r: Ratio::from_u8(5),
                g: Ratio::from_u8(10),
                b: Ratio::from_u8(15),
                a: Ratio::from_u8(255),
            }
        );
        assert_eq!(
            HSL::new(6, 93, 71),
            HSL {
                h: Angle::new(6),
                s: Ratio::from_percentage(93),
                l: Ratio::from_percentage(71)
            }
        );
        assert_eq!(
            HSLA::new(6, 93, 71, 255),
            HSLA {
                h: Angle::new(6),
                s: Ratio::from_percentage(93),
                l: Ratio::from_percentage(71),
                a: Ratio::from_u8(255),
            }
        );
    }

    #[macro_use]
    mod conversions {
        use crate::{Angle, Ratio, HSL, HSLA, RGB, RGBA};

        pub trait ApproximatelyEq {
            fn approximately_eq(self, other: Self) -> bool;
        }

        impl ApproximatelyEq for u8 {
            fn approximately_eq(self, other: Self) -> bool {
                self == other || self + 1 == other || self - 1 == other
            }
        }

        impl ApproximatelyEq for u16 {
            fn approximately_eq(self, other: Self) -> bool {
                self == other || self + 1 == other || self - 1 == other
            }
        }

        impl ApproximatelyEq for Angle {
            fn approximately_eq(self, other: Self) -> bool {
                self.degrees().approximately_eq(other.degrees())
            }
        }

        impl ApproximatelyEq for Ratio {
            fn approximately_eq(self, other: Self) -> bool {
                self.as_u8().approximately_eq(other.as_u8())
            }
        }

        impl ApproximatelyEq for RGB {
            fn approximately_eq(self, other: Self) -> bool {
                self.r.approximately_eq(other.r)
                    && self.g.approximately_eq(other.g)
                    && self.b.approximately_eq(other.b)
            }
        }

        impl ApproximatelyEq for RGBA {
            fn approximately_eq(self, other: Self) -> bool {
                self.r.approximately_eq(other.r)
                    && self.g.approximately_eq(other.g)
                    && self.b.approximately_eq(other.b)
                    && self.a == other.a
            }
        }

        impl ApproximatelyEq for HSL {
            fn approximately_eq(self, other: Self) -> bool {
                self.h.approximately_eq(other.h)
                    && self
                        .s
                        .as_percentage()
                        .approximately_eq(other.s.as_percentage())
                    && self
                        .l
                        .as_percentage()
                        .approximately_eq(other.l.as_percentage())
            }
        }

        impl ApproximatelyEq for HSLA {
            fn approximately_eq(self, other: Self) -> bool {
                self.h.approximately_eq(other.h)
                    && self
                        .s
                        .as_percentage()
                        .approximately_eq(other.s.as_percentage())
                    && self
                        .l
                        .as_percentage()
                        .approximately_eq(other.l.as_percentage())
                    && self.a == other.a
            }
        }

        #[macro_export]
        macro_rules! assert_approximately_eq {
            ($lhs:expr, $rhs:expr) => {
                let lhs = $lhs;
                let rhs = $rhs;

                assert!(lhs.approximately_eq(rhs), "lhs: {}, rhs: {}", lhs, rhs);
            };
        }

        macro_rules! conversion_test {
            (
                $color_name:ident,
                rgb($r:expr, $g:expr, $b:expr),
                hsl($h:expr, $s:expr, $l:expr)
            ) => {
                mod $color_name {
                    use super::ApproximatelyEq;
                    use $crate::{Color, HSL, HSLA, RGB, RGBA};

                    #[test]
                    fn rgb_to_rgb() {
                        assert_eq!(RGB::new($r, $g, $b).to_rgb(), RGB::new($r, $g, $b));
                    }

                    #[test]
                    fn rgb_to_rgba() {
                        assert_eq!(RGB::new($r, $g, $b).to_rgba(), RGBA::new($r, $g, $b, 255));
                    }

                    #[test]
                    fn rgba_to_rgb() {
                        assert_eq!(RGBA::new($r, $g, $b, 255).to_rgb(), RGB::new($r, $g, $b));
                        assert_eq!(RGBA::new($r, $g, $b, 200).to_rgb(), RGB::new($r, $g, $b));
                        assert_eq!(RGBA::new($r, $g, $b, 0).to_rgb(), RGB::new($r, $g, $b));
                    }

                    #[test]
                    fn rgba_to_rgba() {
                        assert_eq!(
                            RGBA::new($r, $g, $b, 255).to_rgba(),
                            RGBA::new($r, $g, $b, 255)
                        );

                        assert_eq!(
                            RGBA::new($r, $g, $b, 200).to_rgba(),
                            RGBA::new($r, $g, $b, 200)
                        );

                        assert_eq!(RGBA::new($r, $g, $b, 0).to_rgba(), RGBA::new($r, $g, $b, 0));
                    }

                    #[test]
                    fn rgb_to_hsl() {
                        assert_approximately_eq!(
                            RGB::new($r, $g, $b).to_hsl(),
                            HSL::new($h, $s, $l)
                        );
                    }

                    #[test]
                    fn rgb_to_hsla() {
                        assert_approximately_eq!(
                            RGB::new($r, $g, $b).to_hsla(),
                            HSLA::new($h, $s, $l, 255)
                        );
                    }

                    #[test]
                    fn rgba_to_hsl() {
                        assert_approximately_eq!(
                            RGBA::new($r, $g, $b, 255).to_hsl(),
                            HSL::new($h, $s, $l)
                        );

                        assert_approximately_eq!(
                            RGBA::new($r, $g, $b, 200).to_hsl(),
                            HSL::new($h, $s, $l)
                        );

                        assert_approximately_eq!(
                            RGBA::new($r, $g, $b, 0).to_hsl(),
                            HSL::new($h, $s, $l)
                        );
                    }

                    #[test]
                    fn rgba_to_hsla() {
                        assert_approximately_eq!(
                            RGBA::new($r, $g, $b, 255).to_hsla(),
                            HSLA::new($h, $s, $l, 255)
                        );

                        assert_approximately_eq!(
                            RGBA::new($r, $g, $b, 200).to_hsla(),
                            HSLA::new($h, $s, $l, 200)
                        );

                        assert_approximately_eq!(
                            RGBA::new($r, $g, $b, 0).to_hsla(),
                            HSLA::new($h, $s, $l, 0)
                        );
                    }

                    #[test]
                    fn hsl_to_hsl() {
                        assert_eq!(HSL::new($h, $s, $l).to_hsl(), HSL::new($h, $s, $l));
                    }

                    #[test]
                    fn hsl_to_hsla() {
                        assert_eq!(HSL::new($h, $s, $l).to_hsla(), HSLA::new($h, $s, $l, 255));
                    }

                    #[test]
                    fn hsla_to_hsl() {
                        assert_eq!(HSLA::new($h, $s, $l, 255).to_hsl(), HSL::new($h, $s, $l));

                        assert_eq!(HSLA::new($h, $s, $l, 200).to_hsl(), HSL::new($h, $s, $l));

                        assert_eq!(HSLA::new($h, $s, $l, 0).to_hsl(), HSL::new($h, $s, $l));
                    }

                    #[test]
                    fn hsla_to_hsla() {
                        assert_eq!(
                            HSLA::new($h, $s, $l, 255).to_hsla(),
                            HSLA::new($h, $s, $l, 255)
                        );

                        assert_eq!(
                            HSLA::new($h, $s, $l, 200).to_hsla(),
                            HSLA::new($h, $s, $l, 200)
                        );

                        assert_eq!(HSLA::new($h, $s, $l, 0).to_hsla(), HSLA::new($h, $s, $l, 0));
                    }

                    #[test]
                    fn hsl_to_rgb() {
                        assert_approximately_eq!(
                            HSL::new($h, $s, $l).to_rgb(),
                            RGB::new($r, $g, $b)
                        );
                    }

                    #[test]
                    fn hsl_to_rgba() {
                        assert_approximately_eq!(
                            HSL::new($h, $s, $l).to_rgba(),
                            RGBA::new($r, $g, $b, 255)
                        );
                    }

                    #[test]
                    fn hsla_to_rgb() {
                        assert_approximately_eq!(
                            HSLA::new($h, $s, $l, 255).to_rgb(),
                            RGB::new($r, $g, $b)
                        );

                        assert_approximately_eq!(
                            HSLA::new($h, $s, $l, 200).to_rgb(),
                            RGB::new($r, $g, $b)
                        );

                        assert_approximately_eq!(
                            HSLA::new($h, $s, $l, 0).to_rgb(),
                            RGB::new($r, $g, $b)
                        );
                    }

                    #[test]
                    fn hsla_to_rgba() {
                        assert_approximately_eq!(
                            HSLA::new($h, $s, $l, 255).to_rgba(),
                            RGBA::new($r, $g, $b, 255)
                        );

                        assert_approximately_eq!(
                            HSLA::new($h, $s, $l, 200).to_rgba(),
                            RGBA::new($r, $g, $b, 200)
                        );

                        assert_approximately_eq!(
                            HSLA::new($h, $s, $l, 0).to_rgba(),
                            RGBA::new($r, $g, $b, 0)
                        );
                    }
                }
            };
        }

        conversion_test!(black, rgb(0, 0, 0), hsl(0, 0, 0));
        conversion_test!(grey, rgb(230, 230, 230), hsl(0, 0, 90));
        conversion_test!(white, rgb(255, 255, 255), hsl(0, 0, 100));
        conversion_test!(pink, rgb(253, 216, 229), hsl(339, 90, 92));
        conversion_test!(brown, rgb(172, 96, 83), hsl(9, 35, 50));
        conversion_test!(teal, rgb(23, 98, 119), hsl(193, 68, 28));
        conversion_test!(green, rgb(89, 161, 54), hsl(100, 50, 42));
        conversion_test!(pale_blue, rgb(148, 189, 209), hsl(200, 40, 70));
        conversion_test!(mauve, rgb(136, 102, 153), hsl(280, 20, 50));
        conversion_test!(cherry, rgb(230, 25, 60), hsl(350, 80, 50));
        conversion_test!(tomato, rgb(255, 99, 71), hsl(9, 100, 64));
        conversion_test!(light_salmon, rgb(255, 160, 122), hsl(17, 100, 74));
        conversion_test!(blue_violet, rgb(138, 43, 226), hsl(271, 76, 53));
        conversion_test!(dark_orange, rgb(255, 140, 0), hsl(33, 100, 50));
        conversion_test!(deep_pink, rgb(255, 20, 147), hsl(328, 100, 54));
        conversion_test!(chartreuse, rgb(127, 255, 0), hsl(90, 100, 50));
    }

    #[test]
    fn can_saturate() {
        use self::conversions::ApproximatelyEq;

        assert_eq!(HSL::new(9, 35, 50).saturate(20), HSL::new(9, 55, 50));
        assert_eq!(
            HSLA::new(9, 35, 50, 255).saturate(20),
            HSLA::new(9, 55, 50, 255)
        );

        assert_approximately_eq!(RGB::new(172, 96, 83).saturate(20), RGB::new(197, 78, 57));
        assert_approximately_eq!(
            RGBA::new(172, 96, 83, 255).saturate(20),
            RGBA::new(197, 78, 57, 255)
        );
    }

    #[test]
    fn can_desaturate() {
        use self::conversions::ApproximatelyEq;

        assert_eq!(HSL::new(9, 55, 50).desaturate(20), HSL::new(9, 35, 50));
        assert_eq!(
            HSLA::new(9, 55, 50, 255).desaturate(20),
            HSLA::new(9, 35, 50, 255)
        );
        assert_approximately_eq!(RGB::new(197, 78, 57).desaturate(20), RGB::new(172, 96, 83));
        assert_approximately_eq!(
            RGBA::new(197, 78, 57, 255).desaturate(20),
            RGBA::new(172, 96, 83, 255)
        );
    }

    #[test]
    fn can_lighten() {
        use self::conversions::ApproximatelyEq;

        assert_eq!(HSL::new(9, 35, 50).lighten(20), HSL::new(9, 35, 70));
        assert_eq!(
            HSLA::new(9, 35, 50, 255).lighten(20),
            HSLA::new(9, 35, 70, 255)
        );
        assert_approximately_eq!(RGB::new(172, 96, 83).lighten(20), RGB::new(205, 160, 152));
        assert_approximately_eq!(
            RGBA::new(172, 96, 83, 255).lighten(20),
            RGBA::new(205, 160, 152, 255)
        );
    }

    #[test]
    fn can_darken() {
        use self::conversions::ApproximatelyEq;

        assert_eq!(HSL::new(9, 35, 70).darken(20), HSL::new(9, 35, 50));
        assert_eq!(
            HSLA::new(9, 35, 70, 255).darken(20),
            HSLA::new(9, 35, 50, 255)
        );
        assert_approximately_eq!(RGB::new(205, 160, 152).darken(20), RGB::new(172, 96, 83));
        assert_approximately_eq!(
            RGBA::new(205, 160, 152, 255).darken(20),
            RGBA::new(172, 96, 83, 255)
        );
    }

    #[test]
    fn can_fadein() {
        assert_eq!(HSL::new(9, 35, 50).fadein(20), HSL::new(9, 35, 50));
        assert_eq!(
            HSLA::new(9, 35, 50, 128).fadein(20),
            HSLA::new(9, 35, 50, 148)
        );
        assert_eq!(RGB::new(172, 96, 83).fadein(20), RGB::new(172, 96, 83));
        assert_eq!(
            RGBA::new(172, 96, 83, 128).fadein(20),
            RGBA::new(172, 96, 83, 148)
        );
    }

    #[test]
    fn can_fadeout() {
        assert_eq!(HSL::new(9, 35, 50).fadeout(20), HSL::new(9, 35, 50));
        assert_eq!(RGB::new(172, 96, 83).fadeout(20), RGB::new(172, 96, 83));
        assert_eq!(
            HSLA::new(9, 35, 50, 148).fadeout(20),
            HSLA::new(9, 35, 50, 128)
        );
        assert_eq!(
            RGBA::new(172, 96, 83, 148).fadeout(20),
            RGBA::new(172, 96, 83, 128)
        );
    }

    #[test]
    fn can_fade() {
        let faded_color = RGBA::new(23, 98, 119, 50);

        assert_eq!(RGB::new(23, 98, 119).fade(50), faded_color);
        assert_eq!(RGBA::new(23, 98, 119, 255).fade(50), faded_color);
        assert_eq!(HSL::new(193, 67, 28).fade(50), faded_color.to_hsla());
        assert_eq!(HSLA::new(193, 67, 28, 255).fade(50), faded_color.to_hsla());
    }

    #[test]
    fn can_spin_forward() {
        use self::conversions::ApproximatelyEq;

        assert_approximately_eq!(RGB::new(75, 207, 23).spin(100), RGB::new(23, 136, 207));
        assert_approximately_eq!(
            RGBA::new(75, 207, 23, 255).spin(100),
            RGB::new(23, 136, 207)
        );
        assert_approximately_eq!(HSL::new(10, 90, 50).spin(30), RGB::new(242, 166, 13));
        assert_approximately_eq!(HSLA::new(10, 90, 50, 255).spin(30), RGB::new(242, 166, 13));
    }

    #[test]
    fn can_spin_backwards() {
        use self::conversions::ApproximatelyEq;

        assert_approximately_eq!(RGB::new(75, 207, 23).spin(-100), RGB::new(207, 32, 23));
        assert_approximately_eq!(
            RGBA::new(75, 207, 23, 255).spin(-100),
            RGB::new(207, 32, 23)
        );
        assert_approximately_eq!(HSL::new(10, 90, 50).spin(-30), RGB::new(242, 13, 89));
        assert_approximately_eq!(HSLA::new(10, 90, 50, 255).spin(-30), RGB::new(242, 13, 89));
    }

    #[test]
    fn can_mix() {
        use self::conversions::ApproximatelyEq;

        let red = RGBA::new(100, 0, 0, 255);
        let green = RGBA::new(0, 100, 0, 255);
        let brown = RGBA::new(50, 50, 0, 255);

        assert_approximately_eq!(red.mix(green, 50), brown);
    }

    #[test]
    fn can_mix_single_color() {
        use self::conversions::ApproximatelyEq;

        let red = RGBA::new(100, 0, 0, 255);
        let green = RGBA::new(0, 100, 0, 127);

        assert_approximately_eq!(red.mix(green, 100), red);
        assert_approximately_eq!(red.mix(green, 0), green);
        assert_approximately_eq!(green.mix(red, 100), green);
        assert_approximately_eq!(green.mix(red, 0), red);
    }

    #[test]
    fn can_mix_with_alpha() {
        use self::conversions::ApproximatelyEq;

        let red = RGBA::new(100, 0, 0, 255);
        let green = RGBA::new(0, 100, 0, 127);
        let brown = RGBA::new(75, 25, 0, 191);

        assert_approximately_eq!(red.mix(green, 50), brown);
        assert_approximately_eq!(green.mix(red, 50), brown);
    }

    #[test]
    fn can_tint() {
        use self::conversions::ApproximatelyEq;

        assert_eq!(
            RGBA::new(0, 0, 255, 128).tint(50),
            RGBA::new(191, 191, 255, 191)
        );
        assert_approximately_eq!(RGB::new(0, 0, 255).tint(50), RGBA::new(128, 128, 255, 255));
        assert_approximately_eq!(HSL::new(6, 93, 71).tint(50), RGBA::new(253, 191, 184, 255));
        assert_approximately_eq!(
            HSLA::new(6, 93, 71, 128).tint(50),
            RGBA::new(254, 223, 219, 191)
        );
    }

    #[test]
    fn can_shade() {
        use self::conversions::ApproximatelyEq;

        assert_eq!(
            RGBA::new(0, 0, 255, 128).shade(50),
            RGBA::new(0, 0, 64, 191)
        );

        assert_approximately_eq!(RGB::new(0, 0, 255).shade(50), RGBA::new(0, 0, 128, 255));

        assert_approximately_eq!(HSL::new(6, 93, 71).shade(50), RGBA::new(125, 63, 56, 255));

        assert_approximately_eq!(
            HSLA::new(6, 93, 71, 128).shade(50),
            RGBA::new(63, 32, 28, 191)
        );
    }

    #[test]
    fn can_greyscale() {
        assert_eq!(RGB::new(128, 242, 13).greyscale(), RGB::new(128, 128, 128));
        assert_eq!(
            RGBA::new(128, 242, 13, 255).greyscale(),
            RGBA::new(128, 128, 128, 255)
        );
        assert_eq!(HSL::new(90, 90, 50).greyscale(), HSL::new(90, 0, 50));
        assert_eq!(
            HSLA::new(90, 90, 50, 255).greyscale(),
            HSLA::new(90, 0, 50, 255)
        );
    }

    #[test]
    fn can_clone() {
        let rgb_color = RGB::new(5, 10, 15);
        let rgba_color = RGBA::new(5, 10, 15, 255);
        let hsl_color = HSL::new(6, 93, 71);
        let hsla_color = HSLA::new(6, 93, 71, 255);

        assert_eq!(rgb_color, rgb_color.clone());
        assert_eq!(rgba_color, rgba_color.clone());
        assert_eq!(hsl_color, hsl_color.clone());
        assert_eq!(hsla_color, hsla_color.clone());
    }

    #[test]
    fn can_copy() {
        let rgb_color = RGB::new(172, 95, 82);
        let rgba_color = RGBA::new(172, 95, 82, 255);
        let hsl_color = HSL::new(9, 35, 50);
        let hsla_color = HSLA::new(9, 35, 50, 255);

        let copied_rgb_color = rgb_color;
        let copied_rgba_color = rgba_color;
        let copied_hsl_color = hsl_color;
        let copied_hsla_color = hsla_color;

        assert_eq!(rgb_color, copied_rgb_color);
        assert_eq!(rgba_color, copied_rgba_color);
        assert_eq!(hsl_color, copied_hsl_color);
        assert_eq!(hsla_color, copied_hsla_color);
    }

    #[test]
    fn can_debug() {
        let rgb_value = format!("{:?}", RGB::new(5, 10, 15));
        let rgba_value = format!("{:?}", RGBA::new(5, 10, 15, 255));
        let hsl_value = format!("{:?}", HSL::new(6, 93, 71));
        let hsla_value = format!("{:?}", HSLA::new(6, 93, 71, 255));

        assert_eq!(rgb_value, "RGB { r: Ratio(5), g: Ratio(10), b: Ratio(15) }");
        assert_eq!(
            rgba_value,
            "RGBA { r: Ratio(5), g: Ratio(10), b: Ratio(15), a: Ratio(255) }"
        );
        assert_eq!(
            hsl_value,
            "HSL { h: Angle { degrees: 6 }, s: Ratio(237), l: Ratio(181) }"
        );
        assert_eq!(
            hsla_value,
            "HSLA { h: Angle { degrees: 6 }, s: Ratio(237), l: Ratio(181), a: Ratio(255) }"
        );
    }

    #[test]
    fn can_convert_to_css() {
        let rgb = RGB::new(5, 10, 255);
        let rgba = RGBA::new(5, 10, 255, 255);
        let hsl = HSL::new(6, 93, 71);
        let hsla = HSLA::new(6, 93, 71, 255);

        assert_eq!(rgb.to_css(), "rgb(5, 10, 255)");
        assert_eq!(rgba.to_css(), "rgba(5, 10, 255, 1.00)");
        assert_eq!(hsl.to_css(), "hsl(6, 93%, 71%)");
        assert_eq!(hsla.to_css(), "hsla(6, 93%, 71%, 1.00)");
    }

    #[test]
    fn can_print_in_css() {
        let printed_rgb = format!("{}", RGB::new(5, 10, 255));
        let printed_rgba = format!("{}", RGBA::new(5, 10, 255, 255));
        let printed_hsl = format!("{}", HSL::new(6, 93, 71));
        let printed_hsla = format!("{}", HSLA::new(6, 93, 71, 255));

        assert_eq!(printed_rgb, "rgb(5, 10, 255)");
        assert_eq!(printed_rgba, "rgba(5, 10, 255, 1.00)");
        assert_eq!(printed_hsl, "hsl(6, 93%, 71%)");
        assert_eq!(printed_hsla, "hsla(6, 93%, 71%, 1.00)");
    }

    #[test]
    fn can_be_displayed() {
        let rgb = RGB::new(5, 10, 255);
        let rgba = RGBA::new(5, 10, 255, 190);
        let hsl = HSL::new(6, 93, 71);
        let hsla = HSLA::new(6, 93, 71, 255);

        assert_eq!("rgb(5, 10, 255)".to_owned(), format!("{}", rgb));
        assert_eq!("rgba(5, 10, 255, 0.75)".to_owned(), format!("{}", rgba));
        assert_eq!("hsl(6, 93%, 71%)".to_owned(), format!("{}", hsl));
        assert_eq!("hsla(6, 93%, 71%, 1.00)".to_owned(), format!("{}", hsla));
    }

    #[test]
    fn can_be_stringified() {
        let rgb = RGB::new(5, 10, 255);
        let rgba = RGBA::new(5, 10, 255, 128);
        let hsl = HSL::new(6, 93, 71);
        let hsla = HSLA::new(6, 93, 71, 128);

        assert_eq!(String::from("rgb(5, 10, 255)"), rgb.to_string());
        assert_eq!(String::from("rgba(5, 10, 255, 0.50)"), rgba.to_string());
        assert_eq!(String::from("hsl(6, 93%, 71%)"), hsl.to_string());
        assert_eq!(String::from("hsla(6, 93%, 71%, 0.50)"), hsla.to_string());
    }
}
