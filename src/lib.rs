use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
/// A struct to represent how much red, green, and blue should be added to create a color.
/// For more, see the [CSS Color Spec](https://www.w3.org/TR/2018/REC-css-color-3-20180619/#rgb-color).
///
pub struct RGB {
    // red between 0-255
    pub r: u8,

    // green
    pub g: u8,

    // blue
    pub b: u8,
}


/// Converts a set of RGB values into valid CSS.
///
/// # Examples
///
/// ```
/// let salmon = css_colors::RGB { r: 250, g: 128, b: 114 };
///
/// assert_eq!("rgb(250, 128, 114)", salmon.to_css());
/// ```
impl RGB {
    pub fn to_css(&self) -> String {
        format!("rgb({red}, {green}, {blue})", red=self.r, green=self.g, blue=self.b)
    }
}

impl fmt::Display for RGB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "rgb({}, {}, {})", self.r, self.g, self.b)
    }
}


#[cfg(test)]
mod rgb_tests {
    use ::RGB;

    #[test]
    fn can_clone() {
        let color = RGB { r: 5, g: 10, b: 15 };

        assert_eq!(color, color.clone());
    }

    #[test]
    fn can_copy() {
        let color = RGB { r: 5, g: 10, b: 15 };
        let copied_color = color;

        assert_eq!(color, copied_color);
    }

    #[test]
    fn can_debug() {
        let color = RGB { r: 5, g: 10, b: 15 };
        let value = format!("{:?}", color);

        assert_eq!(value, "RGB { r: 5, g: 10, b: 15 }");
    }

    #[test]
    fn can_convert_to_css() {
        let color = RGB { r: 5, g: 10, b: 255 };

        assert_eq!(color.to_css(), "rgb(5, 10, 255)");
    }

    #[test]
    fn can_print_in_css() {
        let color = RGB { r: 5, g: 10, b: 255 };
        let printed = format!("{}", color);

        assert_eq!(printed, "rgb(5, 10, 255)");
    }

    #[test]
    fn can_be_displayed() {
        let color = RGB { r: 5, g: 10, b: 255 };

        assert_eq!("rgb(5, 10, 255)".to_owned(), format!("{}", color));
    }

    #[test]
    fn can_be_stringified() {
        let color = RGB { r: 5, g: 10, b: 255 };
        let color_string = String::from("rgb(5, 10, 255)");

        assert_eq!(color_string, color.to_string());
    }
}
