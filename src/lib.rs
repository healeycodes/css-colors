#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rgb {
    // red between 0-255
    pub r: u8,

    // green
    pub g: u8,

    // blue
    pub b: u8,
}

impl Rgb {
    fn to_css(&self) -> String {
        let rgb = format!("rgb({red}, {green}, {blue})", red=self.r, green=self.g, blue=self.b);

        return rgb
    }
}


#[cfg(test)]
mod rgb_tests {
    #[test]
    fn can_clone() {
        let color = ::Rgb { r: 5, g: 10, b: 15 };

        assert_eq!(color, color.clone());
    }

    #[test]
    fn can_copy() {
        let color = ::Rgb { r: 5, g: 10, b: 15 };
        let copied_color = color;

        assert_eq!(color, copied_color);
    }

    #[test]
    fn can_debug() {
        let color = ::Rgb { r: 5, g: 10, b: 15 };
        let value = println!("color: {:?}", color);

        // println!("color: {:?}", "color: Rgb { r: 5, g: 10, b: 15");
        // FIXME: this won't compile
        // assert_eq!(value.to_string(), "color: Rgb { r: 5, g: 10, b: 15 ");
    }

    #[test]
    fn can_convert_to_css() {
        let color = ::Rgb { r: 5, g: 10, b: 15 };

        assert_eq!(color.to_css(), "rgb(5, 10, 15)");
    }
}
